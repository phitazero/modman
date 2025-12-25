use crate::arg_parser::ParsedArgs;
use crate::{LocalMod, Modpack};
use std::process::exit;

pub const EXPLICIT: char = 'e';
pub const DEPENDENCY: char = 'd';
pub const UNREQUIRED: char = 't';

pub fn filter_mods(modpack: &Modpack, parsed_args: &ParsedArgs) -> Vec<LocalMod> {
	let args = &parsed_args.args;

	let mut mods = modpack.mods.clone();

	if args.len() != 0 {
		mods.retain(|m| args.contains(&m.slug));
	}

	if parsed_args.option(DEPENDENCY) && parsed_args.option(EXPLICIT) {
		eprintln!("fatal: filtering for both explicitly installed and dependency mods is contradictory");
		exit(1);
	}

	if parsed_args.option(DEPENDENCY) {
		mods.retain(|m| m.is_dependency);
	} else if parsed_args.option(EXPLICIT) {
		mods.retain(|m| !m.is_dependency);
	}

	if parsed_args.option(UNREQUIRED) {
		mods.retain(|m| modpack.n_dependents(&m.project_id) == 0);
	}

	mods
}
