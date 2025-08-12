mod error;

use diesel::{
	Connection, PgConnection,
	r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub use self::error::{Error, Result};
use crate::config::core_config;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConn = PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!(); // assumes "migrations/" at project root

/// Production database pool (runs migrations automatically)
pub async fn new_db_pool() -> Result<DbPool> {
	let database_url = core_config().DB_URL.clone();

	// Step 1: Establish connection manually to run migrations
	let mut conn = PgConnection::establish(&database_url)
		.map_err(|e| Error::FailToConnect(e.to_string()))?;

	// Step 2: Run migrations using embedded files
	conn.run_pending_migrations(MIGRATIONS)
		.map_err(|e| Error::MigrationError(e.to_string()))?;

	// Step 3: Create pool normally
	let manager = ConnectionManager::<PgConnection>::new(database_url);
	let pool = Pool::builder()
		.build(manager)
		.map_err(|ex| Error::FailToCreatePool(ex.to_string()))?;

	Ok(pool)
}

/// Creates a new isolated test database, runs embedded migrations, and returns a pool and DB name.
pub async fn new_test_db_pool() -> Result<(DbPool, String)> {
	use diesel::RunQueryDsl;
	use uuid::Uuid;

	let base_url = core_config().DB_URL_BASE.clone(); // e.g. "postgres://postgres:welcome@localhost"
	let test_db_name = format!(
		"threadly_test_{}",
		Uuid::new_v4().to_string().replace('-', "_")
	);
	let test_db_url = format!("{}/{}", base_url, test_db_name);

	// Connect to base DB and create new test DB
	let mut admin_conn = PgConnection::establish(&base_url)
		.map_err(|e| Error::FailToConnect(e.to_string()))?;

	diesel::sql_query(format!(r#"CREATE DATABASE "{}""#, test_db_name))
		.execute(&mut admin_conn)
		.map_err(|e| Error::MigrationError(e.to_string()))?;

	// Connect to new test DB and run migrations
	let mut test_conn = PgConnection::establish(&test_db_url)
		.map_err(|e| Error::FailToConnect(e.to_string()))?;

	test_conn
		.run_pending_migrations(MIGRATIONS)
		.map_err(|e| Error::MigrationError(e.to_string()))?;

	// Create pool for test DB
	let manager = ConnectionManager::<PgConnection>::new(test_db_url);
	let pool = Pool::builder()
		.build(manager)
		.map_err(|ex| Error::FailToCreatePool(ex.to_string()))?;

	Ok((pool, test_db_name))
}

/// Drops the given test database — terminate connections and drop DB
pub fn drop_test_db(db_name: &str) -> Result<()> {
	use diesel::RunQueryDsl;

	let base_url = core_config().DB_URL_BASE.clone();

	let mut admin_conn = PgConnection::establish(&base_url)
		.map_err(|e| Error::FailToConnect(e.to_string()))?;

	// Terminate all connections to the DB
	let terminate_sql = format!(
		"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
		db_name
	);
	diesel::sql_query(terminate_sql)
		.execute(&mut admin_conn)
		.map_err(|e| Error::MigrationError(e.to_string()))?;

	// Drop the database
	let drop_sql = format!(r#"DROP DATABASE "{}";"#, db_name);
	diesel::sql_query(drop_sql)
		.execute(&mut admin_conn)
		.map_err(|e| Error::MigrationError(e.to_string()))?;

	Ok(())
}
