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

	let config_file = match config_path.exists() {
		true => {
			let config_file = match File::open(config_path) {
				Ok(file) => file,
				Err(_) => {
					eprintln!("error: couldn't open config file");
					exit(1);
				}
			};
			config_file
		},
		false => {
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

			config_file
		},
	};

	config_file
}
