use crate::{Modpack, ResultTracker};
use crate::cli::SyncArgs;

pub fn command_sync(args: SyncArgs) {
	let mut modpack = Modpack::require_current();

	let mut result_tracker = ResultTracker::new();

	for slug in &args.args {
		let result = modpack.install_by_slug(
			slug,
			!args.no_deps,
		);

		result_tracker.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't install mod {slug}");
			eprintln!("error: {err}");
		}

		eprint!("\n");
	}

	result_tracker.summarize();
}
