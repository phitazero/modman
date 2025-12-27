use super::filter::{filter_mods_by_options, EXPLICIT, DEPENDENCY, UNREQUIRED};
use crate::Modpack;
use crate::arg_parser::ParsedArgs;

pub fn command_query(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['Q', 'q', EXPLICIT, DEPENDENCY, UNREQUIRED]);

	let modpack = Modpack::require_current();
	let mods = filter_mods_by_options(&modpack, &parsed_args);

	for local_mod in mods {
		print!("{}", local_mod.slug);

		if !parsed_args.option('q') {
			print!(" {}", local_mod.version_number);
		}

		print!("\n");
	}
}
