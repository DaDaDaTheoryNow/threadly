use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use lib_core::model::{schema::sessions, schema_enums::SessionStatus};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, AsChangeset, Clone, Serialize)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = sessions)]
pub struct Session {
	pub id: Uuid,
	pub theme: String,
	pub status: SessionStatus,
	pub current_player_id_turn: Option<Uuid>,
	pub max_rounds: i32,
	pub current_round: i32,
	pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
	pub theme: &'a str,
	pub max_rounds: i32,
}
