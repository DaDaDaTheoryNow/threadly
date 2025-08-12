use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::sessions;
use uuid::Uuid;

use crate::model::NewSession;
use crate::model::Session;

impl HasTable for Session {
	type Table = sessions::table;
	fn table() -> Self::Table {
		sessions::table
	}
}

impl BasicDbOps for Session {
	type Id = Uuid;
	type Insert<'a> = NewSession<'a>;

	fn create<'a>(
		conn: &mut PgConnection,
		item: NewSession<'a>,
	) -> QueryResult<Self> {
		diesel::insert_into(Self::table())
			.values(item)
			.get_result(conn)
	}

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self> {
		sessions::table.find(id).get_result(conn)
	}

	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
		sessions::table.load(conn)
	}

	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self> {
		diesel::update(sessions::table.find(id))
			.set(changes)
			.get_result(conn)
	}

	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize> {
		diesel::delete(sessions::table.find(id)).execute(conn)
	}
}
