use std::sync::Arc;

use crate::{
	app_state::AppState,
	error::{AppError, Result},
};
use axum::{
	extract::{Request, State},
	http::{HeaderMap, Uri},
	middleware::Next,
	response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use lib_auth::{auth::jwt::Claims, users::model::User};
use lib_core::{
	config::core_config,
	ctx::{Ctx, CtxExtError, CtxExtResult, Result as CoreResult},
	model::{ModelManager, base::BasicDbOps},
};
use tracing::debug;

pub async fn mw_required_auth(
	ctx: CoreResult<Ctx>,
	request: Request,
	next: Next,
) -> Result<Response> {
	debug!("{:<12} - mw_required_auth", "MIDDLEWARE");

	// TODO: add another errors handling
	ctx.map_err(|_| AppError::CtxExt(CtxExtError::CtxNotInRequestExt))?;

	Ok(next.run(request).await)
}

pub async fn mw_ctx_resolver(
	State(app_state): State<AppState>,
	headers: HeaderMap,
	mut request: Request,
	next: Next,
) -> Response {
	debug!("{:<12} - mw_ctx_resolver", "MIDDLEWARE");

	let ctx_ext_result =
		_ctx_resolve(app_state.model_manager, &headers, request.uri()).await;
	request.extensions_mut().insert(ctx_ext_result);

	next.run(request).await
}

async fn _ctx_resolve(
	mm: Arc<ModelManager>,
	headers: &HeaderMap,
	uri: &Uri,
) -> CtxExtResult {
	let mut db = mm.db();

	let token = extract_bearer_token(headers)
		.or_else(|| extract_token_from_query(uri))
		.ok_or(CtxExtError::TokenNotInBarier)?;

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

fn extract_token_from_query(uri: &Uri) -> Option<String> {
	uri.query().and_then(|q| {
		form_urlencoded::parse(q.as_bytes())
			.find(|(k, _)| k == "token")
			.map(|(_, v)| v.into_owned())
	})
}
