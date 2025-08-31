use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Token error: {0}")]
	Token(#[from] crate::token_manager::error::Error),

	#[error("Reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),

	#[error("SerdeJson error: {0}")]
	SerdeJson(#[from] serde_json::Error),

	#[error("Stream error: {0}")]
	Stream(#[from] reqwest_streams::error::StreamBodyError),
}
