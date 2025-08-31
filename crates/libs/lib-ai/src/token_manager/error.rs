use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Google auth error: {0}")]
	GoogleAuth(#[from] crate::google_auth::error::Error),

	#[error("Reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("Other AI error: {0}")]
	Other(String),
}
