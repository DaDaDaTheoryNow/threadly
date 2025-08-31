use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSessionPayload {
	pub theme: String,
	pub max_rounds: i32,
}

#[derive(Deserialize)]
pub struct JoinSessionPayload {
	pub session_id: Uuid,
}

#[derive(Deserialize)]
pub struct LeaveSessionPayload {
	pub session_id: Uuid,
}

#[derive(Deserialize)]
pub struct ReadyPayload {
	pub session_id: Uuid,
	pub is_ready: bool,
}

#[derive(Deserialize)]
pub struct StartGamePayload {
	pub session_id: Uuid,
}

#[derive(Deserialize)]
pub struct SubmitMessagePayload {
	pub session_id: Uuid,
	pub content: String,
}
