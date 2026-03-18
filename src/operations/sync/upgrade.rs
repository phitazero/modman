use crate::{Modpack, ResultTracker};
use crate::cli::SyncArgs;
use crate::cli::Filters;

pub fn command_upgrade(args: SyncArgs) {
	let mut modpack = Modpack::require_current();

	let slugs: Vec<(String, bool)> = modpack.filter_mods(
		&args.args,
		&Filters {
			dependencies: false,
			explicit: true,
			unrequired: false
		},
	).into_iter()
		.map(|m| (m.slug, m.auto_install_deps))
		.collect();

	let mut result_tracker_remove = ResultTracker::new();
	let mut result_tracker_sync = ResultTracker::new();

	for (slug, _) in &slugs {
		let result = modpack.remove(slug, true);

		result_tracker_remove.register(slug, &result);

		if let Err(err) = result {
			eprintln!("Couldn't remove mod {slug}");
			eprintln!("error: {err}");
		}

		eprint!("\n");
	}

	for (slug, install_deps) in &slugs {
		let result = modpack.install_by_slug(slug, *install_deps);

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
