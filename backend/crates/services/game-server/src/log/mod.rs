use axum::http::{Method, Uri};
use lib_core::{client_error::ClientError, ctx::CtxExtResult};
use tracing::debug;
use uuid::Uuid;

pub async fn log_request(
	uuid: Uuid,
	req_method: Method,
	uri: Uri,
	ctx: Option<CtxExtResult>,
	client_error: Option<&ClientError>,
) -> Result<(), ()> {
	let ctx_str = match ctx.as_ref() {
		Some(Ok(c)) => format!("user_id={}, username={}", c.user_id, c.username),
		_ => "no_ctx".to_string(),
	};

	debug!(
		"REQ_LOG | uuid={} | method={} | uri={} | ctx={} | client_error={:?}",
		uuid, req_method, uri, ctx_str, client_error
	);

	Ok(())
}
