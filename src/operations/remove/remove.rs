use crate::{Modpack, ResultTracker};
use crate::cli::RemoveArgs;

pub fn command_remove(args: RemoveArgs) {
	let mut modpack = Modpack::require_current();

	let mut result_tracker = ResultTracker::new();

	for slug in &args.args {
		let result = modpack.remove(
			slug,
			args.remove_dependencies,
		);

		result_tracker.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't remove mod {slug}");
			eprintln!("error: {err}");
		}

		eprint!("\n");
	}

	result_tracker.summarize();
}
