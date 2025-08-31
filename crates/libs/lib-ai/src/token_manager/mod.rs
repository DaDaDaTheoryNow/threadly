pub mod error;

use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use error::{Error, Result};

pub struct TokenManager {
	key_path: String,
	token: RwLock<Option<(String, Instant)>>,
}

impl TokenManager {
	pub fn new(key_path: &str) -> Self {
		Self {
			key_path: key_path.to_string(),
			token: RwLock::new(None),
		}
	}

	pub async fn get_token(&self) -> Result<String> {
		{
			let token_guard = self.token.read().await;
			if let Some((tok, exp)) = &*token_guard {
				if Instant::now() < *exp {
					return Ok(tok.clone());
				}
			}
		}

		let new_token = self.refresh_token().await?;
		Ok(new_token)
	}

	async fn refresh_token(&self) -> Result<String> {
		// get new token from google_auth
		let token = crate::google_auth::get_access_token(&self.key_path)
			.await
			.map_err(Error::GoogleAuth)?;

		// token lives 1 hour, leave a margin of 55 minutes
		let expiry = Instant::now() + Duration::from_secs(55 * 60);

		let mut token_guard = self.token.write().await;
		*token_guard = Some((token.clone(), expiry));

		Ok(token)
	}
}
