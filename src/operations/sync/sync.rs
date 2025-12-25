use crate::{Modpack, Version, ResultTracker};
use crate::arg_parser::ParsedArgs;
use std::process::exit;

pub fn command_sync(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S']);

	let args = parsed_args.args;

	if args.len() == 0 {
		eprintln!("fatal: no target specified");
		exit(1);
	}

	let mut modpack = Modpack::require_current();

	let mut result_tracker = ResultTracker::new();

	for slug in args.iter() {
		let result = install_by_slug(slug, &mut modpack);

		result_tracker.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't install mod {slug}");
			eprintln!("error: {err}");
		}

		print!("\n");
	}

	result_tracker.summarize();
}

fn install_by_slug(slug: &str, modpack: &mut Modpack) -> Result<(), String> {
	let version = Version::fetch_latest(slug, modpack)?;
	modpack.install(version)
}
