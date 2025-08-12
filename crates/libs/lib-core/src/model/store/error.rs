use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	FailToCreatePool(String),
	FailToConnect(String),
	MigrationError(String),
}
