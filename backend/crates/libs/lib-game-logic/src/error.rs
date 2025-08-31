use std::sync::Arc;

use axum::http::StatusCode;
use derive_more::From;
use lib_core::client_error::ClientError;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, Error, Clone)]
pub enum Error {
	#[error("Database error: {0}")]
	#[from(diesel::result::Error)]
	DbError(#[serde_as(as = "DisplayFromStr")] Arc<diesel::result::Error>),

	#[error("UUID parse error: {0}")]
	#[from(uuid::Error)]
	UuidParseError(#[serde_as(as = "DisplayFromStr")] uuid::Error),

	#[error("Session is not ready")]
	SessionNotReady,

	#[error("Game is already finished")]
	AlreadyFinished,

	#[error("Not enough players")]
	NotEnoughPlayers,

	#[error("Not enough rounds")]
	NotEnoughRounds,

	#[error("Game has already started")]
	AlreadyStarted,

	#[error("Player not found")]
	PlayerNotFound,

	#[error("Invalid turn")]
	InvalidTurn,

	#[error("Player already joined the session")]
	AlreadyJoined,

	#[error("Action allowed only for host")]
	NotHost,

	#[error("User is not in session")]
	UserNotInSession,

	#[error("Unknown error occurred")]
	Unknown,

	#[error("AI generation error: {0}")]
	AiGenerationError(String),
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			Error::SessionNotReady => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::AlreadyFinished => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::NotEnoughPlayers => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::NotEnoughRounds => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::AlreadyStarted => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::PlayerNotFound => (
				StatusCode::NOT_FOUND,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::InvalidTurn => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::AlreadyJoined => (
				StatusCode::BAD_REQUEST,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::NotHost => (
				StatusCode::FORBIDDEN,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::UserNotInSession => (
				StatusCode::FORBIDDEN,
				ClientError::GAME_ERROR(self.to_string()),
			),
			Error::Unknown => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::GAME_ERROR("Session not found".to_string()),
			),
		}
	}
}
