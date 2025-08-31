use lib_core::dto::session::UserInSessionDto;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SessionEvent {
	Created {
		session_id: Uuid,
		theme: String,
		max_rounds: i32,
		users: Vec<UserInSessionDto>,
	},

	UpdatePlayers {
		session_id: Uuid,
		users: Vec<UserInSessionDto>,
	},

	Started {
		session_id: Uuid,
	},

	Deleted {
		session_id: Uuid,
	},
}
