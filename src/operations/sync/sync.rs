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

	for slug in args {
		let result = install_by_slug(&slug, &mut modpack);

		if let Err(err) = result {
			eprintln!("Couldn't install mod {slug}");
			eprintln!("error: {err}");
		}
	}
}

fn install_by_slug(slug: &str, modpack: &mut Modpack) -> Result<(), String> {
	let version = Version::fetch_latest(slug, modpack)?;
	modpack.install(version)
}
