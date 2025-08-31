use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};
use lib_core::client_error::ClientError;
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, AuthError>;

#[derive(Debug, thiserror::Error, Serialize, Clone)]
pub enum AuthError {
	#[error("Invalid token")]
	InvalidToken,
	#[error("User not found")]
	UserNotFound,
	#[error("Email or username already exists")]
	UserExists,
	#[error("Database error: {0}")]
	DbError(String),
	#[error("Password hashing error: {0}")]
	HashError(String),
	#[error("Password verification error: {0}")]
	VerifyError(String),
	#[error("JWT creation error: {0}")]
	JwtError(String),
}

impl AuthError {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			AuthError::InvalidToken => (
				StatusCode::UNAUTHORIZED,
				ClientError::AUTHENTICATION_FAILED("Invalid token".to_string()),
			),
			AuthError::UserNotFound => (
				StatusCode::NOT_FOUND,
				ClientError::AUTHENTICATION_FAILED("User not found".to_string()),
			),
			AuthError::UserExists => (
				StatusCode::BAD_REQUEST,
				ClientError::AUTHENTICATION_FAILED("User exists".to_string()),
			),
			AuthError::DbError(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			AuthError::HashError(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			AuthError::VerifyError(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			AuthError::JwtError(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
		}
	}
}

impl IntoResponse for AuthError {
	fn into_response(self) -> Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(self);

		response
	}
}
