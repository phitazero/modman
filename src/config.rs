use serde::{Serialize, Deserialize};
use directories::BaseDirs;
use std::process::exit;
use std::fs::{self, File};

const DEFAULT_CONFIG: Config = Config {
	mods_search_limit: Some(5)
};

#[derive(Serialize, Deserialize)]
pub struct Config {
	mods_search_limit: Option<u8>,
}
