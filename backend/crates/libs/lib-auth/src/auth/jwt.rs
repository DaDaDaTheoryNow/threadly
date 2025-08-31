use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: Uuid,
	pub exp: usize,
}

pub fn create_jwt(
	user_id: Uuid,
	secret: &[u8],
) -> Result<String, jsonwebtoken::errors::Error> {
	let expiration = Utc::now()
		.checked_add_signed(Duration::hours(1))
		.expect("valid timestamp")
		.timestamp() as usize;

	let claims = Claims {
		sub: user_id,
		exp: expiration,
	};

	jsonwebtoken::encode(
		&Header::default(),
		&claims,
		&EncodingKey::from_secret(secret),
	)
}
