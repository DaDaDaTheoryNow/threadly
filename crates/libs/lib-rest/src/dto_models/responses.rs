use chrono::NaiveDateTime;
use lib_core::{dto::session::UserInSessionDto, model::schema_enums::SessionStatus};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct SessionResponse {
	pub session_id: Uuid,
	pub host_user_id: Uuid,
}

#[derive(Serialize)]
pub struct PlayerResponse {
	pub user_id: Uuid,
	pub is_ready: bool,
	pub is_host: bool,
}

#[derive(Serialize)]
pub struct SessionWithUsersDto {
	pub id: Uuid,
	pub theme: String,
	pub status: SessionStatus,
	pub current_user_id_turn: Option<Uuid>,
	pub max_rounds: i32,
	pub current_round: i32,
	pub created_at: NaiveDateTime,
	pub users: Vec<UserInSessionDto>,
}

impl From<lib_sessions::model::SessionWithUsersInSession> for SessionWithUsersDto {
	fn from(model: lib_sessions::model::SessionWithUsersInSession) -> Self {
		Self {
			id: model.id,
			theme: model.theme,
			status: model.status,
			current_user_id_turn: model.current_user_id_turn,
			max_rounds: model.max_rounds,
			current_round: model.current_round,
			created_at: model.created_at,
			users: model
				.users
				.into_iter()
				.map(|user| UserInSessionDto {
					user_id: user.user_id,
					is_ready: user.is_ready,
					is_host: user.is_host,
				})
				.collect(),
		}
	}
}
