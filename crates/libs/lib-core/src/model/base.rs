use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub trait BasicDbOps: Sized + HasTable {
	type Id: Eq;

	type Insert<'a>: Insertable<<Self as HasTable>::Table> + 'a;

	fn create<'a>(
		conn: &mut PgConnection,
		item: Self::Insert<'a>,
	) -> QueryResult<Self>;

	fn get(conn: &mut PgConnection, id: Self::Id) -> QueryResult<Self>;
	fn list(conn: &mut PgConnection) -> QueryResult<Vec<Self>>;
	fn update(
		conn: &mut PgConnection,
		id: Self::Id,
		changes: &Self,
	) -> QueryResult<Self>;
	fn delete(conn: &mut PgConnection, id: Self::Id) -> QueryResult<usize>;
}
