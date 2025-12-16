use serde::{Serialize, Deserialize};
use std::fs::File;
use std::process::exit;
use crate::utils;
use crate::MANIFEST_FILENAME;
use crate::LocalMod;

#[derive(Debug)]
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

	pub fn current() -> Option<Modpack> {
		let path = utils::current_dir()
			.join(MANIFEST_FILENAME);

		match File::open(path) {
			Ok(file) => {
				match serde_json::from_reader::<File, Modpack>(file) {
					Ok(modpack) => Some(modpack),
					Err(_) => {
						// not clarifying the error here because the manifest is not to edit manually
						eprintln!("error: couldn't read manifest JSON");
						exit(1);
					}
				}
			},
			Err(err) => {
				if err.kind() == std::io::ErrorKind::NotFound {
					None
				} else {
					eprintln!("error: failed to open modpack manifest");
					exit(1);
				}
			}
		}
	}

	pub fn require_current() -> Modpack {
		Modpack::current().unwrap_or_else(|| {
			eprintln!("error: modpack required");
			exit(1);
		})
	}
}
