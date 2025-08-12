use std::sync::Arc;

use crate::{
	app_state::AppState,
	error::{AppError, Result},
};
use axum::{
	extract::{Request, State},
	http::HeaderMap,
	middleware::Next,
	response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use lib_auth::{auth::jwt::Claims, users::model::User};
use lib_core::{
	config::core_config,
	ctx::{Ctx, CtxExtError, CtxExtResult},
	model::{ModelManager, base::BasicDbOps},
};
use tracing::debug;

pub async fn mw_required_auth(request: Request, next: Next) -> Result<Response> {
	debug!("{:<12} - mw_required_auth", "MIDDLEWARE");

	let ctx_result = request
		.extensions()
		.get::<CtxExtResult>()
		.cloned()
		.ok_or(AppError::CtxExt(CtxExtError::CtxNotInRequestExt))?;

	ctx_result?;

	Ok(next.run(request).await)
}

pub async fn mw_ctx_resolver(
	State(app_state): State<AppState>,
	headers: HeaderMap,
	mut request: Request,
	next: Next,
) -> Response {
	debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

	let ctx_ext_result = _ctx_resolve(app_state.model_manager, &headers).await;
	request.extensions_mut().insert(ctx_ext_result);

	next.run(request).await
}

async fn _ctx_resolve(mm: Arc<ModelManager>, headers: &HeaderMap) -> CtxExtResult {
	let mut db = mm.db();

	let token =
		extract_bearer_token(headers).ok_or(CtxExtError::TokenNotInBarier)?;

	let token_data = decode::<Claims>(
		&token,
		&DecodingKey::from_secret(core_config().JWT_SECRET.as_bytes()),
		&Validation::default(),
	)
	.map_err(|_| CtxExtError::FailValidate)?;

	let user_id = token_data.claims.sub;
	let user = User::get(&mut db, user_id)
		.map_err(|disel_error| CtxExtError::DbError(disel_error.to_string()))?;

	Ctx::new(user_id, user.username)
		.map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
	headers
		.get("authorization")
		.and_then(|hv| hv.to_str().ok())
		.and_then(|s| s.strip_prefix("Bearer "))
		.map(|s| s.to_string())
}
