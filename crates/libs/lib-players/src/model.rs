use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use lib_core::model::schema::players;
use uuid::Uuid;

#[derive(Debug, Queryable, AsChangeset, Clone)]
#[diesel(table_name = players)]
pub struct Player {
	pub id: Uuid,
	pub session_id: Uuid,
	pub user_id: Uuid,
	pub joined_at: NaiveDateTime,
	pub is_ready: bool,
	pub is_host: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = players)]
pub struct NewPlayer {
	pub session_id: Uuid,
	pub user_id: Uuid,
	pub is_ready: bool,
	pub is_host: bool,
}
