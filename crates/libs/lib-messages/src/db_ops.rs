use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::messages;
use uuid::Uuid;

use crate::model::Message;
use crate::model::NewMessage;

impl HasTable for Message {
	type Table = messages::table;
	fn table() -> Self::Table {
		messages::table
	}
}

impl BasicDbOps for Message {
	type Id = Uuid;
	type Insert<'a> = NewMessage<'a>;

	fn create<'a>(
		conn: &mut PgConnection,
		item: NewMessage<'a>,
	) -> QueryResult<Self> {
		diesel::insert_into(Self::table())
			.values(item)
			.get_result(conn)
	}

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self> {
		messages::table.find(id).get_result(conn)
	}

	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
		messages::table.load(conn)
	}

	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self> {
		diesel::update(messages::table.find(id))
			.set(changes)
			.get_result(conn)
	}

	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize> {
		diesel::delete(messages::table.find(id)).execute(conn)
	}
}
