use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSessionPayload {
	pub theme: String,
	pub max_rounds: i32,
}

#[derive(Deserialize)]
pub struct ReadyPayload {
	pub player_id: Uuid,
	pub ready: bool,
}

#[derive(Deserialize)]
pub struct StartGamePayload {
	pub host_player_id: Uuid,
}

#[derive(Deserialize)]
pub struct SubmitMessagePayload {
	pub player_id: Uuid,
	pub content: String,
}

#[derive(Serialize)]
pub struct SessionResponse {
	pub session_id: Uuid,
	pub player_id: Uuid,
}

#[derive(Serialize)]
pub struct PlayerResponse {
	pub player_id: Uuid,
	pub is_ready: bool,
	pub is_host: bool,
}
