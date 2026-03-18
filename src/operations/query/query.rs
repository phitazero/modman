use crate::Modpack;
use crate::cli::query::{Filters, QueryListArgs};

pub fn command_query(args: QueryListArgs, filters: Filters) {
	let modpack = Modpack::require_current();
	let mods = modpack.filter_mods(
		&args.slugs,
		&filters
	);

	for local_mod in mods {
		print!("{}", local_mod.slug);

		if !args.quiet {
			print!(" {}", local_mod.version_number);
		}

		print!("\n");
	}
}
