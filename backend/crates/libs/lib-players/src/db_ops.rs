use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::players;
use uuid::Uuid;

use crate::model::{NewPlayer, Player, PlayerId};

impl HasTable for Player {
	type Table = players::table;

	fn table() -> Self::Table {
		players::table
	}
}

impl BasicDbOps for Player {
	type Id = PlayerId;
	type Insert<'a> = NewPlayer;

	fn create<'a>(conn: &mut PgConnection, item: NewPlayer) -> QueryResult<Self> {
		diesel::insert_into(Self::table())
			.values(&item)
			.get_result(conn)
	}

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self> {
		Self::table()
			.find((id.session_id, id.user_id))
			.get_result(conn)
	}

	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
		Self::table().load(conn)
	}

	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self> {
		diesel::update(Self::table().find((id.session_id, id.user_id)))
			.set(changes)
			.get_result(conn)
	}

	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize> {
		diesel::delete(Self::table().find((id.session_id, id.user_id))).execute(conn)
	}
}

impl Player {
	pub fn list_by_session(
		conn: &mut PgConnection,
		sid: Uuid,
	) -> QueryResult<Vec<Self>> {
		Self::table()
			.filter(players::session_id.eq(sid))
			.order(players::joined_at.asc())
			.load::<Self>(conn)
	}

	pub fn list_by_user(
		conn: &mut PgConnection,
		uid: Uuid,
	) -> QueryResult<Vec<Self>> {
		Self::table()
			.filter(players::user_id.eq(uid))
			.order(players::joined_at.asc())
			.load::<Self>(conn)
	}
}
