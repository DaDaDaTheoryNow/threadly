use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error),

	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("Reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("JWT error: {0}")]
	Jwt(#[from] jsonwebtoken::errors::Error),

	#[error("Google returned error: {0}")]
	Google(String),
}
