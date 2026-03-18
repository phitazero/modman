use crate::{Modpack, ResultTracker};
use crate::cli::sync::SyncInstallArgs;

pub fn command_sync(args: SyncInstallArgs, auto_install_deps: bool) {
	let mut modpack = Modpack::require_current();

	let mut result_tracker = ResultTracker::new();

	for slug in &args.slugs {
		let result = modpack.install_by_slug(
			slug,
			auto_install_deps,
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
