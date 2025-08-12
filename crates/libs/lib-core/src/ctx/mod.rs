mod error;

use crate::client_error::ClientError;

pub use self::error::{Error, Result};
use axum::{
	extract::FromRequestParts,
	http::{StatusCode, request::Parts},
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Ctx {
	pub user_id: Uuid,
	pub username: String,
}

impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx {
			user_id: Uuid::new_v4(),
			username: String::new(),
		}
	}

	pub fn new(user_id: Uuid, username: String) -> Result<Ctx> {
		Ok(Self { user_id, username })
	}
}

impl Ctx {
	pub fn user_id(&self) -> Uuid {
		self.user_id.clone()
	}

	pub fn username(&self) -> String {
		self.username.clone()
	}
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxNotInRequestExt)?
			.clone()
			.map_err(Error::CtxExt)
	}
}

pub type CtxExtResult = core::result::Result<Ctx, CtxExtError>;

#[derive(Debug, Clone, thiserror::Error, Serialize)]
pub enum CtxExtError {
	#[error("Authentication Bearer token not found in header")]
	TokenNotInBarier,
	#[error("Context not found in request extensions")]
	CtxNotInRequestExt,
	#[error("Failed to create context: {0}")]
	CtxCreateFail(String),
	#[error("Db error: {0}")]
	DbError(String),
	#[error("Token validation failed")]
	FailValidate,
}

impl CtxExtError {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			CtxExtError::TokenNotInBarier => (
				StatusCode::UNAUTHORIZED,
				ClientError::AUTHENTICATION_FAILED(
					"Token not found in header".to_string(),
				),
			),
			CtxExtError::CtxNotInRequestExt => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			CtxExtError::CtxCreateFail(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			CtxExtError::DbError(_) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::INTERNAL_SERVER_ERROR,
			),
			CtxExtError::FailValidate => (
				StatusCode::UNAUTHORIZED,
				ClientError::AUTHENTICATION_FAILED(
					"Token validation failed".to_string(),
				),
			),
		}
	}
}
