use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use lib_core::model::base::BasicDbOps;
use lib_core::model::schema::users;
use uuid::Uuid;

use crate::users::model::NewUser;
use crate::users::model::User;

impl HasTable for User {
	type Table = users::table;
	fn table() -> Self::Table {
		users::table
	}
}

impl BasicDbOps for User {
	type Id = Uuid;
	type Insert<'a> = NewUser<'a>;

	fn create<'a>(conn: &mut PgConnection, item: NewUser<'a>) -> QueryResult<Self> {
		diesel::insert_into(Self::table())
			.values(item)
			.get_result(conn)
	}

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self> {
		users::table.find(id).get_result(conn)
	}

	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>> {
		users::table.load(conn)
	}

	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self> {
		diesel::update(users::table.find(id))
			.set(changes)
			.get_result(conn)
	}

	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize> {
		diesel::delete(users::table.find(id)).execute(conn)
	}
}

impl User {
	pub fn find_by_email(
		db: &mut PgConnection,
		email: &str,
	) -> Result<Option<Self>, diesel::result::Error> {
		users::table
			.filter(users::email.eq(email))
			.first::<Self>(db)
			.optional()
	}
}
