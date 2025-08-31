use std::sync::Arc;

use lib_ai::client::AiClient;
use lib_core::{
	dto::session::UserInSessionDto,
	model::{ModelManager, base::BasicDbOps, schema_enums::SessionStatus},
};
use lib_messages::model::{Message, NewMessage};
use lib_players::model::{NewPlayer, Player, PlayerId};
use lib_sessions::model::{NewSession, Session};
use uuid::Uuid;

use crate::{
	error::{Error, Result},
	story_generation_task::spawn_story_generation_task,
};
use lib_game_events::{
	event::{
		game::{GameEvent, GameEventReceiver},
		session::SessionEvent,
	},
	manager::GameEventsManager,
};

#[derive(Clone)]
pub struct GameEngine {
	model_manager: Arc<ModelManager>,
	game_events_manager: Arc<GameEventsManager>,
	pub ai_client: Arc<AiClient>,
}

impl GameEngine {
	pub fn new(
		model_manager: Arc<ModelManager>,
		game_events_manager: Arc<GameEventsManager>,
		ai_client: Arc<AiClient>,
	) -> Self {
		Self {
			model_manager,
			game_events_manager,
			ai_client,
		}
	}

	/// Creates a new session with a host player.
	pub fn create_session(
		&self,
		theme: &str,
		host_user_id: Uuid, // user_id of host user (not player_id)
		max_rounds: i32,
	) -> Result<Session> {
		if max_rounds < 2 {
			return Err(Error::NotEnoughRounds);
		}

		let mut conn = self.model_manager.db();

		let new_session = NewSession { theme, max_rounds };

		let session = Session::create(&mut conn, new_session)?;

		// Create host player linked to session with user_id
		let new_host_player = NewPlayer {
			session_id: session.id,
			user_id: host_user_id,
			is_ready: false,
			is_host: true,
		};

		Player::create(&mut conn, new_host_player)?;

		self.game_events_manager
			.send_session_event(SessionEvent::Created {
				session_id: session.id,
				theme: session.theme.clone(),
				max_rounds: session.max_rounds,
				users: vec![UserInSessionDto {
					user_id: host_user_id,
					is_ready: false,
					is_host: true,
				}],
			});

		Ok(session)
	}

	/// Sets the readiness status of a player (by player record ID).
	pub fn set_ready(
		&self,
		session_id: Uuid,
		user_id: Uuid,
		ready: bool,
	) -> Result<Player> {
		let mut conn = self.model_manager.db();

		let player_id = PlayerId {
			session_id,
			user_id,
		};

		let mut player = Player::get(&mut conn, player_id)?;
		player.is_ready = ready;

		let updated = Player::update(&mut conn, player_id, &player)?;

		self.game_events_manager.send_game_event(
			player.session_id,
			None,
			GameEvent::PlayerReady {
				user_id: player.user_id,
				ready,
			},
		);

		Ok(updated)
	}

	/// Allows a user to join a session (if it's still waiting).
	pub fn join_session(&self, session_id: Uuid, user_id: Uuid) -> Result<Player> {
		let mut conn = self.model_manager.db();
		let session = Session::get(&mut conn, session_id)?;
		if session.status != SessionStatus::Waiting {
			return Err(Error::AlreadyStarted);
		}

		let player_id = PlayerId {
			session_id,
			user_id,
		};

		let player_should_not_found = Player::get(&mut conn, player_id);
		if player_should_not_found.is_ok() {
			return Err(Error::AlreadyJoined);
		}

		let new_player = NewPlayer {
			session_id,
			user_id,
			is_ready: false,
			is_host: false,
		};

		let player = Player::create(&mut conn, new_player)?;

		self.game_events_manager.send_game_event(
			session_id,
			None,
			GameEvent::PlayerJoined {
				user_id: player.user_id,
			},
		);

		self.game_events_manager
			.send_session_event(SessionEvent::UpdatePlayers {
				session_id: session.id,
				users: Session::list_users_in_session(&mut conn, session_id)?,
			});

		Ok(player)
	}

	/// Allows a user to leave the session.
	/// Handles mid-game player leaving:
	/// - Removes player by player.id
	/// - Ends game if less than 2 players remain during started game
	/// - Advances turn if the leaving player had the current turn
	pub fn leave_session(&self, session_id: Uuid, user_id: Uuid) -> Result<()> {
		let mut conn = self.model_manager.db();

		let player_id = PlayerId {
			session_id,
			user_id,
		};

		let player = Player::get(&mut conn, player_id);
		if player.is_err() {
			return Err(Error::UserNotInSession);
		}

		let mut session = Session::get(&mut conn, session_id)?;

		// Disallow leave if session is already finished
		if session.status == SessionStatus::Finished {
			return Err(Error::AlreadyFinished);
		}

		Player::delete(&mut conn, player_id)?;

		let users_for_session =
			Session::list_users_in_session(&mut conn, session_id)?;

		if (users_for_session.is_empty() || player.unwrap().is_host)
			&& session.status == SessionStatus::Waiting
		{
			Session::delete(&mut conn, session_id)?;

			self.game_events_manager.send_game_event(
				session_id,
				None,
				GameEvent::SessionDeleted,
			);

			self.game_events_manager
				.send_session_event(SessionEvent::Deleted { session_id });
		}

		self.game_events_manager.send_game_event(
			session_id,
			None,
			GameEvent::PlayerLeft { user_id },
		);

		self.game_events_manager
			.send_session_event(SessionEvent::UpdatePlayers {
				session_id: session.id,
				users: users_for_session.clone(),
			});

		// If game started and less than 2 players remain, finish the game
		if session.status == SessionStatus::Started && users_for_session.len() < 2 {
			session.status = SessionStatus::Finished;
			session.current_user_id_turn = None;
			Session::update(&mut conn, session.id, &session)?;
			return Ok(());
		}

		// If the leaving player had the current turn, advance turn to next player
		if session.current_user_id_turn == Some(player_id.user_id) {
			let last_message = Message::get_last_by_session(&mut conn, session_id)?;
			self.next_turn(session_id, last_message.content)?;
		}

		Ok(())
	}

	/// Checks if the session can be started:
	/// - Status must be Waiting
	/// - At least 2 players joined
	/// - All players are ready
	/// - The starter player is host
	pub fn can_start(
		&self,
		session_id: Uuid,
		host_player_id: PlayerId,
	) -> Result<bool> {
		let mut conn = self.model_manager.db();
		let session = Session::get(&mut conn, session_id)?;
		if session.status != SessionStatus::Waiting {
			return Err(Error::AlreadyStarted);
		}

		let players = Player::list_by_session(&mut conn, session_id)?;
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
			.find(|p| p.user_id == host_player_id.user_id)
			.ok_or(Error::PlayerNotFound)?;

		if !starter_player.is_host {
			return Err(Error::NotHost);
		}

		Ok(true)
	}

	/// TODO: improve json request body
	/// Starts the game session:
	/// - Starter player must be host
	/// - Sets status to Started
	/// - Sets current round to 1
	/// - Sets the current turn to the first player's player_id
	pub fn start_game(
		&self,
		session_id: Uuid,
		host_user_id: Uuid,
	) -> Result<Session> {
		let mut conn = self.model_manager.db();

		let host_player_id = PlayerId {
			session_id,
			user_id: host_user_id,
		};

		Player::get(&mut conn, host_player_id)
			.map_err(|_| Error::UserNotInSession)?;

		self.can_start(session_id, host_player_id)?;

		let mut session = Session::get(&mut conn, session_id)?;
		session.status = SessionStatus::Started;
		session.current_round = 1;
		session.current_user_id_turn = Some(host_user_id);

		let updated = Session::update(&mut conn, session.id, &session)?;

		self.game_events_manager
			.send_session_event(SessionEvent::Started {
				session_id: session.id,
			});

		self.game_events_manager.send_game_event(
			session_id,
			None,
			GameEvent::GameStarted,
		);

		self.game_events_manager.send_game_event(
			session_id,
			None,
			GameEvent::NewTurn {
				user_id: session.current_user_id_turn.unwrap(),
			},
		);

		Ok(updated)
	}

	/// Advances the turn to the next player in the session.
	/// - If last player had the turn, increases round.
	/// - Ends game if max rounds reached.
	pub fn next_turn(
		&self,
		session_id: Uuid,
		last_player_message: String,
	) -> Result<()> {
		let mut conn = self.model_manager.db();

		let mut session = Session::get(&mut conn, session_id)?;

		let players = Player::list_by_session(&mut conn, session_id)?;
		if players.is_empty() {
			return Err(Error::NotEnoughPlayers);
		}

		// Find index of current player
		let current_index = session
			.current_user_id_turn
			.and_then(|uid| players.iter().position(|p| p.user_id == uid))
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
				// Max rounds reached — wait for story generation
				session.status = SessionStatus::WaitingForStoryGeneration;
				session.current_user_id_turn = None;
			} else {
				// Start new round, first player gets turn
				session.current_user_id_turn = Some(players[0].user_id);
			}
		} else {
			// Advance turn to next player
			session.current_user_id_turn = Some(players[next_index].user_id);
		}

		Session::update(&mut conn, session.id, &session)?;

		if session.current_user_id_turn.is_some()
			&& session.status == SessionStatus::Started
		{
			self.game_events_manager.send_game_event(
				session_id,
				None,
				GameEvent::NewTurn {
					user_id: session.current_user_id_turn.unwrap(),
				},
			);

			self.game_events_manager.send_game_event(
				session_id,
				Some(GameEventReceiver {
					user_id: session.current_user_id_turn.unwrap(),
				}),
				GameEvent::LastPlayerMessage {
					content: last_player_message,
				},
			);
		} else if session.status == SessionStatus::WaitingForStoryGeneration {
			self.game_events_manager.send_game_event(
				session_id,
				None,
				GameEvent::WaitingForStoryGeneration,
			);

			if let Some(generation_guard) =
				self.ai_client.try_acquire_generation(session_id)
			{
				// Spawn async generation pipeline — does not block current thread
				spawn_story_generation_task(
					session_id,
					generation_guard,
					self.model_manager.clone(),
					self.ai_client.clone(),
					self.game_events_manager.clone(),
				);
			} else {
				return Ok(());
			};
		}

		Ok(())
	}

	/// Checks if it's the given player's turn (by player_id).
	pub fn is_player_turn(&self, session_id: Uuid, user_id: Uuid) -> Result<bool> {
		let mut conn = self.model_manager.db();

		let session = Session::get(&mut conn, session_id)?;
		Ok(session.current_user_id_turn == Some(user_id))
	}

	/// Submits a message (a move) by the player if it's their turn.
	/// Then advances the turn.
	pub fn submit_message(
		&self,
		session_id: Uuid,
		user_id: Uuid,
		content: &str,
	) -> Result<()> {
		// Check if it's the player's turn
		if !Self::is_player_turn(&self, session_id, user_id)? {
			return Err(Error::InvalidTurn);
		}

		let mut conn = self.model_manager.db();

		let session = Session::get(&mut conn, session_id)?;
		let players = Player::list_by_session(&mut conn, session_id)?;

		// Find player's turn order (index)
		let turn_index = players
			.iter()
			.position(|p| p.user_id == user_id)
			.ok_or(Error::PlayerNotFound)?;

		let new_message = NewMessage {
			session_id,
			user_id,
			content,
			round: session.current_round,
			turn_order: turn_index as i32,
		};

		Message::create(&mut conn, new_message.clone())?;

		// Advance the turn after message submission
		self.next_turn(session_id, new_message.content.to_string())?;

		Ok(())
	}
}
