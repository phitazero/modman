use crate::Modpack;
use crate::cli::QueryArgs;

pub fn command_query(args: QueryArgs) {
	let modpack = Modpack::require_current();
	let mods = modpack.filter_mods(
		&args.args,
		&args.filters
	);

	for local_mod in mods {
		print!("{}", local_mod.slug);

		if !args.quiet {
			print!(" {}", local_mod.version_number);
		}

		print!("\n");
	}
}
