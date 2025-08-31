// #[cfg(test)]
// mod test_play_flow {
// 	use std::sync::Arc;

// 	use lib_core::model::{
// 		TestModelManager, base::BasicDbOps, schema_enums::SessionStatus,
// 	};
// 	use lib_game_events::manager::GameEventsManager;
// 	use lib_game_logic::engine::GameEngine;
// 	use lib_players::model::Player;
// 	use lib_sessions::model::Session;
// 	use serial_test::serial;
// 	use uuid::Uuid;

// 	#[tokio::test]
// 	#[serial]
// 	async fn test_play_flow_with_leave() {
// 		let mm = TestModelManager::new()
// 			.await
// 			.expect("Failed to create ModelManager");

// 		let mut conn = mm.db();

// 		let game_events_manager = Arc::new(GameEventsManager::new());
// 		let game_engine = GameEngine::new(game_events_manager.clone());

// 		let user1 = Uuid::new_v4();
// 		let user2 = Uuid::new_v4();

// 		let session = game_engine
// 			.create_session(&mut conn, "test_play_flow_with_leave", user1, 3)
// 			.expect("Failed to create session");

// 		assert_eq!(session.theme, "test_play_flow_with_leave");
// 		assert_eq!(session.max_rounds, 3);
// 		assert_eq!(session.status, SessionStatus::Waiting);

// 		// Get host player from DB (to get player_id)
// 		let players = Player::list_by_session(&mut conn, session.id)
// 			.expect("Failed to list players");
// 		assert_eq!(players.len(), 1);
// 		let host_player = &players[0];
// 		assert_eq!(host_player.user_id, user1);
// 		assert!(host_player.is_host);

// 		// Second user joins the session
// 		let player2 = game_engine
// 			.join_session(&mut conn, session.id, user2)
// 			.expect("User2 failed to join session");

// 		assert_eq!(player2.user_id, user2);
// 		assert!(!player2.is_host);

// 		// Set both players ready
// 		let host_player_ready = game_engine
// 			.set_ready(&mut conn, host_player.id, true)
// 			.expect("Failed to set host ready");
// 		let player2_ready = game_engine
// 			.set_ready(&mut conn, player2.id, true)
// 			.expect("Failed to set player2 ready");
// 		assert!(host_player_ready.is_ready);
// 		assert!(player2_ready.is_ready);

// 		// Check that game can be started
// 		assert!(
// 			GameEngine::can_start(&mut conn, session.id, host_player.id)
// 				.expect("Failed can_start")
// 		);

// 		// Start the game
// 		let session_started = game_engine
// 			.start_game(&mut conn, session.id, host_player.id)
// 			.expect("Failed to start game");
// 		assert_eq!(session_started.status, SessionStatus::Started);
// 		assert_eq!(session_started.current_round, 1);
// 		assert_eq!(session_started.current_player_id_turn, Some(host_player.id));

// 		// Check who's turn it is (should be host)
// 		assert!(
// 			GameEngine::is_player_turn(&mut conn, session.id, host_player.id)
// 				.expect("Failed is_player_turn")
// 		);

// 		// Host makes a move
// 		game_engine
// 			.submit_message(&mut conn, session.id, host_player.id, "host move")
// 			.expect("Host failed to submit message");

// 		// After the move, it should be player2's turn
// 		assert!(
// 			GameEngine::is_player_turn(&mut conn, session.id, player2.id)
// 				.expect("Failed is_player_turn for player2")
// 		);

// 		// Player2 makes a move
// 		game_engine
// 			.submit_message(&mut conn, session.id, player2.id, "player2 move")
// 			.expect("Player2 failed to submit message");

// 		// Turn returned to host, round increased or not (depends on number of players)
// 		let session_after_turns =
// 			Session::get(&mut conn, session.id).expect("Failed to get session");
// 		assert_eq!(
// 			session_after_turns.current_player_id_turn,
// 			Some(host_player.id)
// 		);
// 		assert_eq!(session_after_turns.current_round, 2);

// 		// Player2 leaves the session
// 		game_engine
// 			.leave_session(&mut conn, session.id, player2.id)
// 			.expect("Failed to leave session");

// 		// After player2 leaves, the game should end (because there are less than 2 players)
// 		let session_after_leave = Session::get(&mut conn, session.id)
// 			.expect("Failed to get session after leave");

// 		assert_eq!(session_after_leave.status, SessionStatus::Finished);
// 		assert!(session_after_leave.current_player_id_turn.is_none());

// 		// Drop test DB
// 		mm.drop_db().await.expect("Failed to drop test DB");
// 	}

// 	#[tokio::test]
// 	#[serial]
// 	async fn test_play_flow_full_game() {
// 		let mm = TestModelManager::new()
// 			.await
// 			.expect("Failed to create ModelManager");

// 		let mut conn = mm.db();

// 		let game_events_manager = Arc::new(GameEventsManager::new());
// 		let game_engine = GameEngine::new(game_events_manager.clone());

// 		// Create two users
// 		let user1 = Uuid::new_v4();
// 		let user2 = Uuid::new_v4();

// 		// Create game session (user1 will be host)
// 		let session = game_engine
// 			.create_session(&mut conn, "test_play_flow_full_game", user1, 3)
// 			.expect("Failed to create session");

// 		assert_eq!(session.status, SessionStatus::Waiting);

// 		// Get host player
// 		let players = Player::list_by_session(&mut conn, session.id).unwrap();
// 		let host = &players[0];
// 		assert_eq!(host.user_id, user1);
// 		assert!(host.is_host);

// 		// Second player joins
// 		let player2 = game_engine
// 			.join_session(&mut conn, session.id, user2)
// 			.unwrap();

// 		// Both players ready
// 		game_engine.set_ready(&mut conn, host.id, true).unwrap();
// 		game_engine.set_ready(&mut conn, player2.id, true).unwrap();

// 		// Host starts the game
// 		assert!(GameEngine::can_start(&mut conn, session.id, host.id).unwrap());
// 		let mut session = game_engine
// 			.start_game(&mut conn, session.id, host.id)
// 			.unwrap();
// 		assert_eq!(session.status, SessionStatus::Started);
// 		assert_eq!(session.current_round, 1);

// 		// Play until all rounds are completed
// 		while session.status == SessionStatus::Started {
// 			let current_player_id = session
// 				.current_player_id_turn
// 				.expect("No current player turn");

// 			let message = if current_player_id == host.id {
// 				"host move"
// 			} else if current_player_id == player2.id {
// 				"player2 move"
// 			} else {
// 				panic!("Unexpected current_player_id");
// 			};

// 			game_engine
// 				.submit_message(&mut conn, session.id, current_player_id, message)
// 				.expect("Failed to submit message");

// 			// Refresh session
// 			session = Session::get(&mut conn, session.id).unwrap();
// 		}

// 		// Game should be finished
// 		assert_eq!(session.status, SessionStatus::Finished);
// 		assert_eq!(session.current_round, 4); // round increments AFTER last move
// 		assert!(session.current_player_id_turn.is_none());

// 		// Drop test DB
// 		mm.drop_db().await.expect("Failed to drop test DB");
// 	}
// }
