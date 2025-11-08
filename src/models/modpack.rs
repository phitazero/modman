use serde::{Serialize, Deserialize};
use std::fs::File;
use std::process::exit;
use crate::utils;
use crate::MANIFEST_FILENAME;

#[derive(Serialize, Deserialize)]
pub struct Modpack {
	pub loader: String,
	pub version: String,
	pub mods: Vec<LocalMod>,
}

impl Modpack {
	pub fn save(&self) {
		let path = utils::current_dir()
			.join(MANIFEST_FILENAME);

		match File::create(path) {
			Ok(file) => {
				let result = serde_json::to_writer_pretty(&file, self);

				if result.is_err() {
					eprintln!("error: failed to save modpack manifest");
					exit(1);
				}
			},
			Err(_) => {
				eprintln!("error: failed to open modpack manifest");
				exit(1);
			}
		}
	}
}

// dummy, will be implemented properly in the future
#[derive(Serialize, Deserialize)]
pub struct LocalMod {}
