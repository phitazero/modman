use std::process::exit;

mod arg_parser;
mod requests;
mod operations;
mod config;
mod models;
mod utils;

use config::config;
use models::remote_mod::RemoteMod;
use models::modpack::Modpack;
use models::version::{Version, VersionFile};
use models::local_mod::LocalMod;
use models::result_tracker::ResultTracker;

const MANIFEST_FILENAME: &str = "modman-modpack.json";

fn main() {
	let parsed_args = arg_parser::parse();

	match parsed_args.operation {
		Some('S') => operations::sync::dispatch(parsed_args),
		Some('M') => operations::modpack::dispatch(parsed_args),
		Some('Q') => operations::query::dispatch(parsed_args),
		Some(other) => {
			eprintln!("error: invalid operation \'{}\'", other);
			exit(1);
		},

		None => {
			if parsed_args.options.contains(&'h') {
				todo!("help");
			} else {
				eprintln!("error: no operation specified");
				exit(1);
			}
		}

	}
}
