// use chrono::{DateTime, Utc};
// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};

// use crate::model::{
// 	ModelManager,
// 	base::{DbBmc, DbOps},
// 	error::Result,
// };

// use super::schema::messages;

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
// #[diesel(table_name = messages)]
// pub struct Message {
// 	pub id: i32,
// 	pub content: String,
// 	pub created_at: DateTime<Utc>,
// 	pub updated_at: DateTime<Utc>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct MessageCreate {
// 	pub content: String,
// }

// impl DbBmc for Message {
// 	const TABLE: &'static str = "messages";
// 	type Create = MessageCreate;
// 	type Entity = Message;
// 	type TableType = messages::table;
// }

// impl Message {
// 	pub async fn create(mm: &ModelManager, content: String) -> Result<Message> {
// 		let data = MessageCreate { content };
// 		DbOps::create::<Self>(mm, data).await
// 	}

// 	pub async fn get(mm: &ModelManager, id: i32) -> Result<Message> {
// 		DbOps::get::<Self>(mm, id).await
// 	}

// 	pub async fn list(mm: &ModelManager) -> Result<Vec<Message>> {
// 		DbOps::list::<Self>(mm).await
// 	}

// 	pub async fn update(
// 		mm: &ModelManager,
// 		id: i32,
// 		content: String,
// 	) -> Result<Message> {
// 		let data = MessageCreate { content };
// 		DbOps::update::<Self>(mm, id, data).await
// 	}

// 	pub async fn delete(mm: &ModelManager, id: i32) -> Result<()> {
// 		DbOps::delete::<Self>(mm, id).await
// 	}
// }
