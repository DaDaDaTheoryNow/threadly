use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use lib_core::model::schema::users;
use uuid::Uuid;

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub password_hash: String,
	pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
	pub username: &'a str,
	pub password_hash: &'a str,
}
