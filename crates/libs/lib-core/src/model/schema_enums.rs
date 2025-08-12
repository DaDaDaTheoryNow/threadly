use diesel_derive_enum::DbEnum;
use serde::Serialize;

#[derive(Debug, DbEnum, Clone, PartialEq, Serialize)]
#[ExistingTypePath = "crate::model::schema::sql_types::SessionStatus"]
pub enum SessionStatus {
	#[db_rename = "waiting"]
	Waiting,
	#[db_rename = "started"]
	Started,
	#[db_rename = "finished"]
	Finished,
}
