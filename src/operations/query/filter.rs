use crate::arg_parser::ParsedArgs;
use crate::LocalMod;
use std::process::exit;

pub const EXPLICIT: char = 'e';
pub const DEPENDENCY: char = 'd';

pub fn filter_mods(mods: &mut Vec<LocalMod>, parsed_args: &ParsedArgs) {
	let args = &parsed_args.args;

	if args.len() != 0 {
		mods.retain(|m| args.contains(&m.slug));
	}

	if parsed_args.option(DEPENDENCY) && parsed_args.option(EXPLICIT) {
		eprintln!("error: filtering for both explicitly installed and dependency mods is contradictory");
		exit(1);
	}

	if parsed_args.option(DEPENDENCY) {
		mods.retain(|m| m.is_dependency);
	} else if parsed_args.option(EXPLICIT) {
		mods.retain(|m| !m.is_dependency);
	}
}
