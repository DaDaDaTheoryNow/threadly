use std::sync::Arc;

use diesel::PgConnection;
use lib_core::model::{base::BasicDbOps, schema_enums::SessionStatus};
use lib_messages::model::{Message, NewMessage};
use lib_players::model::{NewPlayer, Player};
use lib_sessions::model::{NewSession, Session};
use uuid::Uuid;

use crate::error::{Error, Result};
use lib_game_events::{
	event::{GameEvent, GameEventReceiver},
	manager::GameEventsManager,
};

#[derive(Clone)]
pub struct GameEngine {
	game_events_manager: Arc<GameEventsManager>,
}

impl GameEngine {
	pub fn new(game_events_manager: Arc<GameEventsManager>) -> Self {
		Self {
			game_events_manager,
		}
	}

	/// Creates a new session with a host player.
	pub fn create_session(
		&self,
		conn: &mut PgConnection,
		theme: &str,
		host_user_id: Uuid, // user_id of host user (not player_id)
		max_rounds: i32,
	) -> Result<Session> {
		if max_rounds < 2 {
			return Err(Error::NotEnoughRounds);
		}

		let new_session = NewSession { theme, max_rounds };

		let session = Session::create(conn, new_session)?;

		// Create host player linked to session with user_id
		let new_host_player = NewPlayer {
			session_id: session.id,
			user_id: host_user_id,
			is_ready: false,
			is_host: true,
		};

		Player::create(conn, new_host_player)?;

		Ok(session)
	}

	/// Sets the readiness status of a player (by player record ID).
	pub fn set_ready(
		&self,
		conn: &mut PgConnection,
		player_id: Uuid, // player record ID
		user_id: Uuid,
		ready: bool,
	) -> Result<Player> {
		let players = Player::list_by_user(conn, user_id)?;
		if !players.iter().any(|p| p.id == player_id) {
			return Err(Error::UserNotInSession);
		}

		let mut player = Player::get(conn, player_id)?;
		player.is_ready = ready;
		let updated = Player::update(conn, player_id, &player)?;

		self.game_events_manager.send(
			player.session_id,
			None,
			GameEvent::PlayerReady {
				player_id: player.id,
				ready,
			},
		);

		Ok(updated)
	}

	/// Allows a user to join a session (if it's still waiting).
	pub fn join_session(
		&self,
		conn: &mut PgConnection,
		session_id: Uuid,
		user_id: Uuid,
	) -> Result<Player> {
		let session = Session::get(conn, session_id)?;
		if session.status != SessionStatus::Waiting {
			return Err(Error::AlreadyStarted);
		}

		// Prevent duplicate join by user_id
		let players = Player::list_by_session(conn, session_id)?;
		if players.iter().any(|p| p.user_id == user_id) {
			return Err(Error::AlreadyJoined);
		}

		let new_player = NewPlayer {
			session_id,
			user_id,
			is_ready: false,
			is_host: false,
		};

		let player = Player::create(conn, new_player)?;

		self.game_events_manager.send(
			session_id,
			None,
			GameEvent::PlayerJoined {
				player_id: player.id,
			},
		);

		Ok(player)
	}

	/// Allows a user to leave the session.
	/// Handles mid-game player leaving:
	/// - Removes player by player.id
	/// - Ends game if less than 2 players remain during started game
	/// - Advances turn if the leaving player had the current turn
	pub fn leave_session(
		&self,
		conn: &mut PgConnection,
		session_id: Uuid,
		player_id: Uuid,
		user_id: Uuid,
	) -> Result<()> {
		let players = Player::list_by_user(conn, user_id)?;
		if !players.iter().any(|p| p.id == player_id) {
			return Err(Error::UserNotInSession);
		}

		let mut session = Session::get(conn, session_id)?;

		// Disallow leave if session is already finished
		if session.status == SessionStatus::Finished {
			return Err(Error::AlreadyFinished);
		}

		let mut players = Player::list_by_session(conn, session_id)?;

		// Find player record by player_id
		let player_to_remove = {
			let player = players
				.iter()
				.find(|p| p.id == player_id)
				.ok_or(Error::PlayerNotFound)?;
			player.clone()
		};

		// Delete player record by player.id
		Player::delete(conn, player_to_remove.id)?;

		// Update players list after deletion
		players.retain(|p| p.id != player_to_remove.id);

		// If game started and less than 2 players remain, finish the game
		if session.status == SessionStatus::Started && players.len() < 2 {
			session.status = SessionStatus::Finished;
			session.current_player_id_turn = None;
			Session::update(conn, session.id, &session)?;
			return Ok(());
		}

		// If the leaving player had the current turn, advance turn to next player
		if session.current_player_id_turn == Some(player_to_remove.id) {
			session.current_player_id_turn = players.first().map(|p| p.id);
			Session::update(conn, session.id, &session)?;
		}

		self.game_events_manager.send(
			session_id,
			None,
			GameEvent::PlayerLeft { player_id },
		);

		Ok(())
	}

	/// Checks if the session can be started:
	/// - Status must be Waiting
	/// - At least 2 players joined
	/// - All players are ready
	/// - The starter player is host
	pub fn can_start(
		conn: &mut PgConnection,
		session_id: Uuid,
		host_player_id: Uuid,
	) -> Result<bool> {
		let session = Session::get(conn, session_id)?;
		if session.status != SessionStatus::Waiting {
			return Err(Error::AlreadyStarted);
		}

		let players = Player::list_by_session(conn, session_id)?;
		if players.len() < 2 {
			return Err(Error::NotEnoughPlayers);
		}

		// Check that all players ready
		let all_ready = players.iter().all(|p| p.is_ready);
		if !all_ready {
			return Err(Error::SessionNotReady);
		}

		// Check that starter_player_id corresponds to host
		let starter_player = players
			.iter()
			.find(|p| p.id == host_player_id)
			.ok_or(Error::PlayerNotFound)?;

		if !starter_player.is_host {
			return Err(Error::NotHost);
		}

		Ok(true)
	}

	/// Starts the game session:
	/// - Starter player must be host
	/// - Sets status to Started
	/// - Sets current round to 1
	/// - Sets the current turn to the first player's player_id
	pub fn start_game(
		&self,
		conn: &mut PgConnection,
		session_id: Uuid,
		host_player_id: Uuid,
		host_user_id: Uuid,
	) -> Result<Session> {
		let players = Player::list_by_user(conn, host_user_id)?;
		if !players.iter().any(|p| p.id == host_player_id) {
			return Err(Error::UserNotInSession);
		}

		Self::can_start(conn, session_id, host_player_id)?;

		let mut session = Session::get(conn, session_id)?;
		session.status = SessionStatus::Started;
		session.current_round = 1;

		let players = Player::list_by_session(conn, session_id)?;
		if let Some(first_player) = players.first() {
			session.current_player_id_turn = Some(first_player.id);
		} else {
			return Err(Error::NotEnoughPlayers);
		}

		let updated = Session::update(conn, session.id, &session)?;

		self.game_events_manager.send(
			session_id,
			None,
			GameEvent::GameStarted { session_id },
		);

		self.game_events_manager.send(
			session_id,
			None,
			GameEvent::NewTurn {
				player_id: session.current_player_id_turn.unwrap(),
			},
		);

		Ok(updated)
	}

	/// Advances the turn to the next player in the session.
	/// - If last player had the turn, increases round.
	/// - Ends game if max rounds reached.
	pub fn next_turn(
		&self,
		conn: &mut PgConnection,
		session_id: Uuid,
		last_player_message: String,
	) -> Result<()> {
		let mut session = Session::get(conn, session_id)?;

		let players = Player::list_by_session(conn, session_id)?;
		if players.is_empty() {
			return Err(Error::NotEnoughPlayers);
		}

		// Find index of current player by player.id
		let current_index = session
			.current_player_id_turn
			.and_then(|pid| players.iter().position(|p| p.id == pid))
			.unwrap_or(usize::MAX);

		let next_index = if current_index == usize::MAX {
			0 // If current player not found, start from first player
		} else {
			current_index + 1
		};

		if next_index >= players.len() {
			// End of round, increment round count
			session.current_round += 1;
			if session.current_round > session.max_rounds {
				// Max rounds reached — finish the game
				session.status = SessionStatus::Finished;
				session.current_player_id_turn = None;
			} else {
				// Start new round, first player gets turn
				session.current_player_id_turn = Some(players[0].id);
			}
		} else {
			// Advance turn to next player
			session.current_player_id_turn = Some(players[next_index].id);
		}

		Session::update(conn, session.id, &session)?;

		if session.current_player_id_turn.is_some()
			&& session.status == SessionStatus::Started
		{
			self.game_events_manager.send(
				session_id,
				None,
				GameEvent::NewTurn {
					player_id: session.current_player_id_turn.unwrap(),
				},
			);

			self.game_events_manager.send(
				session_id,
				Some(GameEventReceiver {
					player_id: session.current_player_id_turn.unwrap(),
				}),
				GameEvent::LastPlayerMessage {
					content: last_player_message,
				},
			);
		} else if session.status == SessionStatus::Finished {
			self.game_events_manager.send(
				session_id,
				None,
				GameEvent::GameFinished { session_id },
			);
		}

		Ok(())
	}

	/// Checks if it's the given player's turn (by player_id).
	pub fn is_player_turn(
		conn: &mut PgConnection,
		session_id: Uuid,
		player_id: Uuid,
	) -> Result<bool> {
		let session = Session::get(conn, session_id)?;
		Ok(session.current_player_id_turn == Some(player_id))
	}

	/// Submits a message (a move) by the player if it's their turn.
	/// Then advances the turn.
	pub fn submit_message(
		&self,
		conn: &mut PgConnection,
		session_id: Uuid,
		player_id: Uuid,
		content: &str,
	) -> Result<()> {
		// Check if it's the player's turn
		if !Self::is_player_turn(conn, session_id, player_id)? {
			return Err(Error::InvalidTurn);
		}

		let session = Session::get(conn, session_id)?;
		let players = Player::list_by_session(conn, session_id)?;

		// Find player's turn order (index)
		let turn_index = players
			.iter()
			.position(|p| p.id == player_id)
			.ok_or(Error::PlayerNotFound)?;

		let user_id = players
			.iter()
			.find(|p| p.id == player_id)
			.ok_or(Error::PlayerNotFound)?
			.user_id;

		let new_message = NewMessage {
			session_id,
			user_id,
			content,
			round: session.current_round,
			turn_order: turn_index as i32,
		};

		Message::create(conn, new_message.clone())?;

		// Advance the turn after message submission
		self.next_turn(conn, session_id, new_message.content.to_string())?;

		Ok(())
	}
}
