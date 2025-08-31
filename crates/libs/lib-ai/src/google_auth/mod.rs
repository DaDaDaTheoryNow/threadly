use chrono::{Duration, Utc};
use error::{Error, Result};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod error;

#[derive(Serialize)]
struct Claims<'a> {
	iss: &'a str,
	scope: &'a str,
	aud: &'a str,
	exp: i64,
	iat: i64,
}

#[derive(Deserialize)]
struct TokenResponse {
	access_token: String,
}

pub async fn get_access_token(sa_key_path: &str) -> Result<String> {
	let data = std::fs::read_to_string(sa_key_path)?;
	let key: Value = serde_json::from_str(&data)?;

	let client_email = key["client_email"]
		.as_str()
		.ok_or_else(|| Error::Google("Missing client_email in key".to_string()))?;
	let private_key = key["private_key"]
		.as_str()
		.ok_or_else(|| Error::Google("Missing private_key in key".to_string()))?;

	// claim for JWT
	let iat = Utc::now();
	let exp = iat + Duration::minutes(55); // token lives ~1 hour
	let claims = Claims {
		iss: client_email,
		scope: "https://www.googleapis.com/auth/cloud-platform",
		aud: "https://oauth2.googleapis.com/token",
		iat: iat.timestamp(),
		exp: exp.timestamp(),
	};

	// sign JWT
	let header = Header::new(Algorithm::RS256);
	let jwt = encode(
		&header,
		&claims,
		&EncodingKey::from_rsa_pem(private_key.as_bytes())?,
	)?;

	// exchange for access_token
	let client = reqwest::Client::new();
	let params = [
		("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
		("assertion", &jwt),
	];

	let resp = client
		.post("https://oauth2.googleapis.com/token")
		.form(&params)
		.send()
		.await?;

	if !resp.status().is_success() {
		let body = resp.text().await.unwrap_or_default();
		return Err(Error::Google(format!("Auth failed: {}", body)));
	}

	let token: TokenResponse = resp.json().await?;
	Ok(token.access_token)
}
