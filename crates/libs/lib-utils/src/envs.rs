use dotenv::dotenv;
use std::{env, str::FromStr};

pub fn get_env(name: &'static str) -> Result<String> {
	dotenv().ok();

	env::var(name).map_err(|_| Error::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	MissingEnv(&'static str),
	WrongFormat(&'static str),
}
