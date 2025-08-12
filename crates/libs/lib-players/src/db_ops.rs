use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::players;
use uuid::Uuid;

use crate::model::NewPlayer;
use crate::model::Player;

impl HasTable for Player {
	type Table = players::table;
	fn table() -> Self::Table {
		players::table
	}
}

impl BasicDbOps for Player {
	type Id = Uuid;
	type Insert<'a> = NewPlayer;

	fn create<'a>(conn: &mut PgConnection, item: NewPlayer) -> QueryResult<Self> {
		diesel::insert_into(Self::table())
			.values(item)
			.get_result(conn)
	}

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self> {
		players::table.find(id).get_result(conn)
	}

	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
		players::table.load(conn)
	}

	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self> {
		diesel::update(players::table.find(id))
			.set(changes)
			.get_result(conn)
	}

	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize> {
		diesel::delete(players::table.find(id)).execute(conn)
	}
}

impl Player {
	pub fn list_by_session(
		conn: &mut PgConnection,
		sid: Uuid,
	) -> QueryResult<Vec<Self>> {
		players::table
			.filter(players::session_id.eq(sid))
			.order(players::joined_at.asc())
			.load::<Self>(conn)
	}

	pub fn list_by_user(
		conn: &mut PgConnection,
		uid: Uuid,
	) -> QueryResult<Vec<Self>> {
		players::table
			.filter(players::user_id.eq(uid))
			.order(players::joined_at.asc())
			.load::<Self>(conn)
	}
}
