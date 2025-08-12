use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	INTERNAL_SERVER_ERROR,
	AUTHENTICATION_FAILED(String),
	GAME_ERROR(String),
}
