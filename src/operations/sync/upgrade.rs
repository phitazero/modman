use crate::{Modpack, ResultTracker};
use crate::arg_parser::ParsedArgs;

pub fn command_upgrade(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S', 'u']);

	let mut modpack = Modpack::require_current();

	let slugs: Vec<String> = modpack.filter_mods(
		&parsed_args.args,
		true, false, false, // only explicitly installed
	).into_iter()
		.map(|m| m.slug)
		.collect();

	let mut result_tracker_remove = ResultTracker::new();
	let mut result_tracker_sync = ResultTracker::new();

	for slug in &slugs {
		let result = modpack.remove(slug, true);

		result_tracker_remove.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't remove mod {slug}");
			eprintln!("error: {err}");
		}

		eprint!("\n");
	}

	for slug in &slugs {
		let result = modpack.install_by_slug(slug);

		result_tracker_sync.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't install mod {slug}");
			eprintln!("error: {err}");
		}

		eprint!("\n");
	}

	eprintln!("Removing:");
	result_tracker_remove.summarize();

	eprint!("\n");

	eprintln!("Sync:");
	result_tracker_sync.summarize();
}
