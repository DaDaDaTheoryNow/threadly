use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};
use lib_core::client_error::ClientError;
use serde::Serialize;
use serde_with::serde_as;
use thiserror::Error;
use tracing::debug;

pub type Result<T> = core::result::Result<T, AppError>;

#[serde_as]
#[derive(Debug, Error, Serialize, Clone)]
pub enum AppError {
	#[error(transparent)]
	AuthError(#[from] lib_auth::auth::error::AuthError),

	#[error(transparent)]
	CtxExt(#[from] lib_core::ctx::CtxExtError),
}

impl AppError {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			AppError::AuthError(err) => err.client_status_and_error(),
			AppError::CtxExt(err) => err.client_status_and_error(),
		}
	}
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		debug!("{:<12} - Error {self:?}", "INTO_RES");

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(self);

		response
	}
}
