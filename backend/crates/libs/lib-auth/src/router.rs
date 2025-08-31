// src/auth/router.rs
use std::sync::Arc;

use axum::{
	Extension, Json, Router, http::StatusCode, response::IntoResponse, routing::post,
};
use lib_core::model::ModelManager;
use serde_json::json;

use crate::auth::{
	dto::{LoginRequest, RegisterRequest},
	error::AuthError,
	service::AuthService,
};

pub fn auth_router(mm: Arc<ModelManager>, jwt_secret: String) -> Router {
	let auth_service = AuthService {
		mm,
		jwt_secret: jwt_secret.clone(),
	};

	Router::new()
		.route("/register", post(register_handler))
		.route("/login", post(login_handler))
		.layer(Extension(auth_service))
}

async fn register_handler(
	Extension(auth_service): Extension<AuthService>,
	Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AuthError> {
	let (token, user_id) = auth_service.register(payload)?;

	Ok((
		StatusCode::OK,
		Json(json!({ "token": token, "user_id": user_id })),
	)
		.into_response())
}

async fn login_handler(
	Extension(auth_service): Extension<AuthService>,
	Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthError> {
	let (token, user_id) = auth_service.login(payload)?;

	Ok((
		StatusCode::OK,
		Json(json!({ "token": token, "user_id": user_id })),
	)
		.into_response())
}
