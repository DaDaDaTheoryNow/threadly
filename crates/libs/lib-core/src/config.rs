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

	// -- AI
	pub GCP_APPLICATION_CREDENTIALS: String,
	pub GCP_PROJECT_ID: String,
	pub GCP_LOCATION: String,
	pub GCP_MODEL_NAME: String,
}

impl CoreConfig {
	fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
		let db_url_base = get_env("DATABASE_URL_BASE")?;
		let db_url = get_env("DATABASE_URL")?;
		let jwt_secret = get_env("JWT_SECRET")?;
		let gcp_application_credentials = get_env("GCP_APPLICATION_CREDENTIALS")?;
		let gcp_project_id = get_env("GCP_PROJECT_ID")?;
		let gcp_location = get_env("GCP_LOCATION")?;
		let gcp_model_name = get_env("GCP_MODEL_NAME")?;

		Ok(CoreConfig {
			// -- Db
			DB_URL_BASE: db_url_base.clone(),
			DB_URL: db_url.clone(),
			// -- JWT
			JWT_SECRET: jwt_secret.clone(),
			// -- AI
			GCP_APPLICATION_CREDENTIALS: gcp_application_credentials.clone(),
			GCP_PROJECT_ID: gcp_project_id.clone(),
			GCP_LOCATION: gcp_location.clone(),
			GCP_MODEL_NAME: gcp_model_name.clone(),
		})
	}
}
