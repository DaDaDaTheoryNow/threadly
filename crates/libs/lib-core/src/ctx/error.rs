use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, thiserror::Error)]
pub enum Error {
	#[error("Context cannot be created for root")]
	CtxCannotNewRootCtx,
	#[error("Context not found in request extensions")]
	CtxNotInRequestExt,

	#[error(transparent)]
	CtxExt(#[from] crate::ctx::CtxExtError),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		(StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
	}
}
