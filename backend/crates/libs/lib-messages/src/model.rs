use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use lib_core::model::schema::messages;
use uuid::Uuid;

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = messages)]
pub struct Message {
	pub id: Uuid,
	pub session_id: Uuid,
	pub user_id: Uuid,
	pub content: String,
	pub round: i32,
	pub turn_order: i32,
	pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
	pub session_id: Uuid,
	pub user_id: Uuid,
	pub content: &'a str,
	pub round: i32,
	pub turn_order: i32,
}
