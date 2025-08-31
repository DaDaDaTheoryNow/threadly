use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct UserInSessionDto {
	pub user_id: Uuid,
	pub is_ready: bool,
	pub is_host: bool,
}
