use std::sync::Arc;

use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
};
use derive_more::From;
use lib_core::client_error::ClientError;

use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};
use tracing::debug;

#[serde_as]
#[derive(Debug, From, Serialize, Clone)]
pub enum Error {
	#[from(lib_game_logic::error::Error)]
	GameEngineError(#[serde_as(as = "DisplayFromStr")] lib_game_logic::error::Error),

	#[from(diesel::result::Error)]
	DbError(#[serde_as(as = "DisplayFromStr")] Arc<diesel::result::Error>),
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			Error::GameEngineError(e) => e.client_status_and_error(),
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!("{:<12} - Error {self:?}", "INTO_RES");

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		response.extensions_mut().insert(self);

		response
	}
}
