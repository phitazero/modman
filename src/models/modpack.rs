use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::process::exit;
use crate::utils;
use crate::config;

const MANIFEST_FILENAME: &str = "modman-modpack.json";

#[derive(Serialize, Deserialize)]
pub struct Modpack {
	#[serde(skip)]
	pub name: String,
	pub loader: String,
	pub version: String,
	pub mods: Vec<LocalMod>,
}

impl Modpack {
	pub fn save(&self) {
		let path = config().get_instances_path()
			.join(&self.name)
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
