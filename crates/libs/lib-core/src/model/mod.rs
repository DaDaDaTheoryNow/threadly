pub mod base;
pub mod error;
pub mod schema;
pub mod schema_enums;

mod store;

use self::error::Result;
use crate::model::store::{
	DbPool, DbPooledConn, drop_test_db, new_db_pool, new_test_db_pool,
};

#[derive(Clone)]
pub struct ModelManager {
	db_pool: DbPool,
}

impl ModelManager {
	/// Creates a ModelManager with the production database
	pub async fn new() -> Result<Self> {
		let pool = new_db_pool().await?;
		Ok(ModelManager { db_pool: pool })
	}

	/// Gets a pooled connection from the database
	pub fn db(&self) -> DbPooledConn {
		self.db_pool.get().expect("Failed to get pooled connection")
	}
}

pub struct TestModelManager {
	pub db_pool: DbPool,
	pub db_name: String,
}

impl TestModelManager {
	/// Creates a fresh test database, runs migrations, returns TestModelManager
	pub async fn new() -> Result<Self> {
		let (pool, db_name) = new_test_db_pool().await?;

		Ok(TestModelManager {
			db_pool: pool,
			db_name,
		})
	}

	/// Gets a pooled connection from the test database
	pub fn db(&self) -> DbPooledConn {
		self.db_pool.get().expect("Failed to get pooled connection")
	}

	/// Drops the test database (call after tests)
	pub async fn drop_db(&self) -> Result<()> {
		drop_test_db(&self.db_name)?;

		Ok(())
	}
}
