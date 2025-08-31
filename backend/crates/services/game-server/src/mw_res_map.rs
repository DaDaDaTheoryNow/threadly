use axum::{
	Json,
	body::Body,
	http::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};
use lib_auth::auth::error::AuthError;
use lib_core::{client_error::ClientError, ctx::CtxExtResult};
use serde_json::{json, to_value};
use tracing::{debug, error};
use uuid::Uuid;

use crate::{error::AppError, log::log_request};

pub async fn mw_response_map(req: Request<Body>, next: Next) -> Response {
	debug!("{:<12} - mw_response_map", "RES_MAPPER");

	let uuid = Uuid::new_v4();
	let uri = req.uri().clone();
	let req_method = req.method().clone();
	let ctx = req.extensions().get::<CtxExtResult>().cloned();

	let res = next.run(req).await;

	let client_status_error = find_client_status_error(&res);

	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_json = to_value(client_error).ok();
				let message =
					client_error_json.as_ref().and_then(|v| v.get("message"));
				let detail =
					client_error_json.as_ref().and_then(|v| v.get("detail"));

				let client_error_body = json!({
					"error": {
						"message": message,
						"data": {
							"req_uuid": uuid.to_string(),
							"detail": detail
						}
					}
				});

				debug!("CLIENT ERROR BODY:\n{client_error_body}");
				(*status_code, Json(client_error_body)).into_response()
			});

	let _ = log_request(
		uuid,
		req_method,
		uri,
		ctx,
		client_status_error.as_ref().map(|(_, err)| err),
	)
	.await;

	debug!("\n");

	error_response.unwrap_or(res)
}

fn find_client_status_error(
	res: &Response,
) -> Option<(axum::http::StatusCode, ClientError)> {
	if let Some(err) = res.extensions().get::<AppError>() {
		error!("APP ERROR: {err}");
		return Some(err.client_status_and_error());
	}
	if let Some(err) = res.extensions().get::<AuthError>() {
		error!("AUTH ERROR: {err}");
		return Some(err.client_status_and_error());
	}
	if let Some(err) = res.extensions().get::<lib_rest::error::Error>() {
		error!("REST ERROR: {err:?}");
		return Some(err.client_status_and_error());
	}

	None
}
