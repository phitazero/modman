use crate::arg_parser::ParsedArgs;
use crate::{Modpack, ResultTracker};
use std::process::exit;

pub fn command_remove(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['R']);

	let args = parsed_args.args;

	if args.len() == 0 {
		eprintln!("fatal: no target specified");
		exit(1);
	}

	let mut modpack = Modpack::require_current();

	let mut result_tracker = ResultTracker::new();

	for slug in args.iter() {
		let result = modpack.remove(slug);

		result_tracker.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't remove mod {slug}");
			eprintln!("error: {err}");
		}

		print!("\n");
	}

	result_tracker.summarize();
}
