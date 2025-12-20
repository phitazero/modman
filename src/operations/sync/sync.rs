use crate::{Modpack, Version};
use crate::arg_parser::ParsedArgs;
use std::process::exit;

pub fn command_sync(parsed_args: ParsedArgs) {
	let args = parsed_args.args;

	if args.len() == 0 {
		eprintln!("error: no target specified");
		exit(1);
	}

	let mut modpack = Modpack::require_current();

	let mut to_install: Vec<(String, Version)> = Vec::new();

	for (slug, version_res) in Version::batch_fetch_latest(args, &modpack) {
		match version_res {
			Ok(version) => to_install.push((slug, version)),
			Err(err) => {
				eprintln!("Couldn't fetch version for {slug}");
				eprintln!("error: {err}");
			}
		}
	}

	for (slug, version) in to_install {
		let result = modpack.install(version);

		if let Err(err) = result {
			eprintln!("Couldn't install mod {slug}");
			eprintln!("error: {err}");
		}
	}
}
