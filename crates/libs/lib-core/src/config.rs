use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
	static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		CoreConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct CoreConfig {
	// -- Db
	pub DB_URL_BASE: String,
	pub DB_URL: String,

	// -- JWT
	pub JWT_SECRET: String,
}

impl CoreConfig {
	fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
		let db_url_base = get_env("DATABASE_URL_BASE")?;
		let db_url = get_env("DATABASE_URL")?;
		let jwt_secret = get_env("JWT_SECRET")?;

		Ok(CoreConfig {
			// -- Db
			DB_URL_BASE: db_url_base.clone(),
			DB_URL: db_url.clone(),
			// -- JWT
			JWT_SECRET: jwt_secret.clone(),
		})
	}
}
