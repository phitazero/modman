use crate::{Modpack};
use crate::arg_parser::ParsedArgs;

pub fn command_query(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['Q', 'd', 'e', 'q']);

	let args = &parsed_args.args;

	let modpack = Modpack::require_current();
	let mut mods = modpack.mods;

	if args.len() != 0 {
		mods.retain(|m| args.contains(&m.slug));
	}

	if parsed_args.option('d') && !parsed_args.option('e') {
		mods.retain(|m| m.is_dependency);
	} else if !parsed_args.option('d') && parsed_args.option('e') {
		mods.retain(|m| !m.is_dependency);
	}

	for local_mod in mods {
		print!("{}", local_mod.slug);

		if !parsed_args.option('q') {
			print!(" {}", local_mod.version_number);
		}

		print!("\n");
	}
}
