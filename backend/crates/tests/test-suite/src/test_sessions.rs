#[cfg(test)]
mod test_super {
	use lib_core::model::{TestModelManager, base::BasicDbOps};
	use lib_sessions::model::{NewSession, Session};
	use serial_test::serial;

	fn sample_session() -> NewSession<'static> {
		NewSession {
			theme: "dark",
			max_rounds: 3,
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_create_get_delete() {
		let mm = TestModelManager::new()
			.await
			.expect("Failed to create ModelManager");
		let mut conn = mm.db();

		// Create
		let session = sample_session();
		let created = Session::create(&mut conn, session).expect("create failed");
		assert_eq!(created.theme, "dark");

		// Get
		let fetched = Session::get(&mut conn, created.id).expect("get failed");
		assert_eq!(fetched.id, created.id);
		assert_eq!(fetched.theme, "dark");

		// Delete
		let deleted = Session::delete(&mut conn, created.id).expect("delete failed");
		assert_eq!(deleted, 1);

		// Confirm deletion by trying to fetch (should error)
		let result = Session::get(&mut conn, created.id);
		assert!(result.is_err());

		// Drop test DB
		mm.drop_db().await.expect("Failed to drop test DB");
	}

	#[tokio::test]
	#[serial]
	async fn test_update() {
		let mm = TestModelManager::new()
			.await
			.expect("Failed to create ModelManager");
		let mut conn = mm.db();

		// Create
		let session = sample_session();
		let created = Session::create(&mut conn, session).expect("create failed");

		// Update theme
		let mut to_update = created;
		to_update.theme = "light".to_string();

		let updated = Session::update(&mut conn, to_update.id, &to_update)
			.expect("update failed");
		assert_eq!(updated.theme, "light");

		// Cleanup
		let deleted = Session::delete(&mut conn, updated.id).expect("delete failed");
		assert_eq!(deleted, 1);

		// Drop test DB
		mm.drop_db().await.expect("Failed to drop test DB");
	}

	#[tokio::test]
	#[serial]
	async fn test_list() {
		let mm = TestModelManager::new()
			.await
			.expect("Failed to create ModelManager");
		let mut conn = mm.db();

		// Create
		let session = sample_session();
		let created = Session::create(&mut conn, session).expect("create failed");

		// List
		let list = Session::list(&mut conn).expect("list failed");
		assert!(list.iter().any(|s| s.id == created.id));

		// Cleanup
		Session::delete(&mut conn, created.id).expect("delete failed");

		// Drop test DB
		mm.drop_db().await.expect("Failed to drop test DB");
	}
}
