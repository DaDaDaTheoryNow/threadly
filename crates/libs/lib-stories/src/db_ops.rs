use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::stories;
use uuid::Uuid;

use crate::model::NewStory;
use crate::model::Story;

impl HasTable for Story {
	type Table = stories::table;
	fn table() -> Self::Table {
		stories::table
	}
}

impl BasicDbOps for Story {
	type Id = Uuid;
	type Insert<'a> = NewStory<'a>;

	fn create<'a>(conn: &mut PgConnection, item: NewStory<'a>) -> QueryResult<Self> {
		diesel::insert_into(Self::table())
			.values(item)
			.get_result(conn)
	}

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self> {
		stories::table.find(id).get_result(conn)
	}

	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
		stories::table.load(conn)
	}

	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self> {
		diesel::update(stories::table.find(id))
			.set(changes)
			.get_result(conn)
	}

	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize> {
		diesel::delete(stories::table.find(id)).execute(conn)
	}
}
