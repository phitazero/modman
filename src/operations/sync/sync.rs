use crate::{Modpack, ResultTracker};
use crate::arg_parser::ParsedArgs;
use std::process::exit;

pub fn command_sync(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S', 'j']);

	let args = &parsed_args.args;

	if args.len() == 0 {
		eprintln!("fatal: no target specified");
		exit(1);
	}

	let mut modpack = Modpack::require_current();

	let mut result_tracker = ResultTracker::new();

	for slug in args {
		let result = modpack.install_by_slug(slug, !parsed_args.option('j'));

		result_tracker.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't install mod {slug}");
			eprintln!("error: {err}");
		}

		eprint!("\n");
	}

	result_tracker.summarize();
}
