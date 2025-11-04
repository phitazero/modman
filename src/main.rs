use std::process::exit;

mod arg_parser;
mod requests;
mod operations;
mod config;
mod modpack;
mod remote_mod;

use config::config;
use remote_mod::RemoteMod;

fn main() {
	let parsed_args = arg_parser::parse();

	match parsed_args.operation {
		Some('S') => operations::sync::dispatch(parsed_args),
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
