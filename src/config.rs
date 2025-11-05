use serde::{Serialize, Deserialize};
use directories::BaseDirs;
use std::process::exit;
use std::fs::{self, File};
use std::path::PathBuf;
use crate::utils;

const DEFAULT_CONFIG: Config = Config {
	mods_search_limit: Some(5), // must always be a Some
	instances_path: None,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
	mods_search_limit: Option<u8>,
	instances_path: Option<String>,
}

impl Config {
	pub fn get_mods_search_limit(&self) -> u8 {
		match self.mods_search_limit {
			Some(value) => value,
			None => DEFAULT_CONFIG.mods_search_limit.unwrap(),
		}
	}

	pub fn get_instances_path(&self) -> PathBuf {
		match self.instances_path.clone() {
			Some(value) => {
				let path = PathBuf::from(value);

				utils::assert_directory(&path);

				path
			},
			None => {
				eprintln!("error: path to instances directory must be defined in config");
				exit(1);
			}
		}
	}
}

pub fn config() -> Config {
	let config_file = open_config();

	match serde_json::from_reader::<File, Config>(config_file) {
		Ok(config) => config,
		Err(error) => {
			eprint!("error: failed to parse config json: ");
			eprint!("{:?} ", error.classify());
			eprintln!("({}:{})", error.line(), error.column());
			exit(1);
		}
	}
}

fn open_config() -> File {
	let config_path = match BaseDirs::new() {
		Some(base_dirs) => base_dirs.config_dir()
			.join("modman")
			.join("config.json"),

		None => {
			eprintln!("error: couldn't get base directories");
			exit(1);
		},
	};

	if let Some(parent) = config_path.parent() {
		if !parent.exists() {
			let result = fs::create_dir_all(parent);

			if result.is_err() {
				eprintln!("error: couldn't create config directory");
				exit(1);
			}
		}
	}

	if !config_path.exists() {
		let config_file = match File::create(&config_path) {
			Ok(file) => file,
			Err(_) => {
				eprintln!("error: couldn't create config file");
				exit(1);
			}
		};

		let result = serde_json::to_writer_pretty(&config_file, &DEFAULT_CONFIG);

		if result.is_err() {
			eprintln!("error: failed to write default config");
			exit(1);
		}
	}

	let config_file = match File::open(config_path) {
		Ok(file) => file,
		Err(_) => {
			eprintln!("error: couldn't open config file");
			exit(1);
		}
	};

	config_file
}
