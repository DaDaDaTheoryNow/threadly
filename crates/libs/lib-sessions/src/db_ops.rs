use std::collections::HashMap;

use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::dto::session::UserInSessionDto;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::{players, sessions};
use uuid::Uuid;

use crate::model::{NewSession, UserInSession};
use crate::model::{Session, SessionWithUsersInSession};

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

impl Session {
	pub fn list_users_in_session(
		conn: &mut PgConnection,
		session_id: Uuid,
	) -> QueryResult<Vec<UserInSessionDto>> {
		players::table
			.filter(players::session_id.eq(session_id))
			.select((players::user_id, players::is_ready, players::is_host))
			.load(conn)
			.map(|players| players.into_iter().collect())
	}

	pub fn list_with_users(
		conn: &mut PgConnection,
	) -> QueryResult<Vec<SessionWithUsersInSession>> {
		let sessions_list: Vec<Session> = sessions::table.get_results(conn)?;

		let players_info: Vec<(Uuid, Uuid, bool, bool)> = players::table
			.select((
				players::session_id,
				players::user_id,
				players::is_ready,
				players::is_host,
			))
			.get_results(conn)?;

		let mut users_map: HashMap<Uuid, Vec<UserInSession>> = HashMap::new();

		for (session_id, user_id, is_ready, is_host) in players_info {
			users_map.entry(session_id).or_insert_with(Vec::new).push(
				UserInSession {
					user_id,
					is_ready,
					is_host,
				},
			);
		}

		let result = sessions_list
			.into_iter()
			.map(|session| {
				let users = users_map.remove(&session.id).unwrap_or_default();

				SessionWithUsersInSession {
					id: session.id,
					theme: session.theme,
					status: session.status,
					current_user_id_turn: session.current_user_id_turn,
					max_rounds: session.max_rounds,
					current_round: session.current_round,
					created_at: session.created_at,
					users,
				}
			})
			.collect();

		Ok(result)
	}

	pub fn get_with_users(
		conn: &mut PgConnection,
		session_id: Uuid,
	) -> QueryResult<Option<SessionWithUsersInSession>> {
		let players_info: Vec<(Uuid, bool, bool)> = players::table
			.filter(players::session_id.eq(session_id))
			.select((players::user_id, players::is_ready, players::is_host))
			.get_results(conn)?;

		let session: Session = sessions::table.find(session_id).get_result(conn)?;

		Ok(Some(SessionWithUsersInSession {
			id: session.id,
			theme: session.theme,
			status: session.status,
			current_user_id_turn: session.current_user_id_turn,
			max_rounds: session.max_rounds,
			current_round: session.current_round,
			created_at: session.created_at,
			users: players_info
				.into_iter()
				.map(|(user_id, is_ready, is_host)| UserInSession {
					user_id,
					is_ready,
					is_host,
				})
				.collect(),
		}))
	}
}
