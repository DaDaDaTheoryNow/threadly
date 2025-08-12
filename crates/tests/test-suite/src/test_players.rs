#[cfg(test)]
mod test_players {
	use lib_core::model::{TestModelManager, base::BasicDbOps};
	use lib_players::model::{NewPlayer, Player};
	use lib_sessions::model::{NewSession, Session};
	use serial_test::serial;
	use uuid::Uuid;

	fn sample_session() -> NewSession<'static> {
		NewSession {
			theme: "test-theme",
			max_rounds: 3,
		}
	}

	fn sample_new_player(session_id: Uuid, user_id: Uuid) -> NewPlayer {
		NewPlayer {
			session_id,
			user_id,
			is_ready: false,
			is_host: true,
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_create_get_delete_player() {
		let mm = TestModelManager::new()
			.await
			.expect("Failed to create ModelManager");
		let mut conn = mm.db();

		let session = Session::create(&mut conn, sample_session())
			.expect("session create failed");

		let user_id = Uuid::new_v4();
		let new_player = sample_new_player(session.id, user_id);
		let player =
			Player::create(&mut conn, new_player).expect("player create failed");

		assert_eq!(player.session_id, session.id);
		assert_eq!(player.user_id, user_id);
		assert_eq!(player.is_ready, false);

		let fetched = Player::get(&mut conn, player.id).expect("player get failed");
		assert_eq!(fetched.id, player.id);

		Player::delete(&mut conn, player.id).expect("player delete failed");

		Session::delete(&mut conn, session.id).expect("session delete failed");

		// Drop test DB
		mm.drop_db().await.expect("Failed to drop test DB");
	}

	#[tokio::test]
	#[serial]
	async fn test_update_player_ready() {
		let mm = TestModelManager::new()
			.await
			.expect("Failed to create ModelManager");
		let mut conn = mm.db();

		let session = Session::create(&mut conn, sample_session())
			.expect("session create failed");
		let user_id = Uuid::new_v4();
		let new_player = sample_new_player(session.id, user_id);

		let mut player =
			Player::create(&mut conn, new_player).expect("player create failed");

		player.is_ready = true;

		let updated = Player::update(&mut conn, player.id, &player)
			.expect("player update failed");

		assert_eq!(updated.is_ready, true);

		Player::delete(&mut conn, player.id).expect("player delete failed");
		Session::delete(&mut conn, session.id).expect("session delete failed");

		// Drop test DB
		mm.drop_db().await.expect("Failed to drop test DB");
	}

	#[tokio::test]
	#[serial]
	async fn test_list_players() {
		let mm = TestModelManager::new()
			.await
			.expect("Failed to create ModelManager");
		let mut conn = mm.db();

		let session = Session::create(&mut conn, sample_session())
			.expect("session create failed");

		let user_id1 = Uuid::new_v4();
		let user_id2 = Uuid::new_v4();

		let player1 =
			Player::create(&mut conn, sample_new_player(session.id, user_id1))
				.expect("player1 create failed");
		let player2 =
			Player::create(&mut conn, sample_new_player(session.id, user_id2))
				.expect("player2 create failed");

		let players = Player::list(&mut conn).expect("list players failed");
		assert!(players.iter().any(|p| p.id == player1.id));
		assert!(players.iter().any(|p| p.id == player2.id));

		Player::delete(&mut conn, player1.id).expect("delete player1 failed");
		Player::delete(&mut conn, player2.id).expect("delete player2 failed");
		Session::delete(&mut conn, session.id).expect("session delete failed");

		// Drop test DB
		mm.drop_db().await.expect("Failed to drop test DB");
	}
}
