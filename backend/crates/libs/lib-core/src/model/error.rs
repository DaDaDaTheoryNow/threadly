use crate::model::store;
use derive_more::From;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
	#[from]
	Store(store::Error),

	#[from]
	Diesel(#[serde_as(as = "DisplayFromStr")] diesel::result::Error),
}
