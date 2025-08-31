use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use lib_core::model::schema::stories;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, AsChangeset, Clone, Serialize)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = stories)]
pub struct Story {
	pub id: Uuid,
	pub session_id: Uuid,
	pub content: String,
	pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = stories)]
pub struct NewStory<'a> {
	pub session_id: Uuid,
	pub content: &'a str,
}
