use crate::arg_parser::ParsedArgs;
use crate::{LocalMod, Modpack};
use std::process::exit;

pub const EXPLICIT: char = 'e';
pub const DEPENDENCY: char = 'd';
pub const UNREQUIRED: char = 't';

pub fn filter_mods(
	modpack: &Modpack,
	args: &Vec<String>,
	explicit: bool,
	dependency: bool,
	unrequired: bool,
) -> Vec<LocalMod> {
	let mut mods = modpack.mods.clone();

	if args.len() != 0 {
		mods.retain(|m| args.contains(&m.slug));
	}

	if dependency && explicit {
		eprintln!("fatal: filtering for both explicitly installed and dependency mods is contradictory");
		exit(1);
	}

	if dependency {
		mods.retain(|m| m.is_dependency);
	} else if explicit {
		mods.retain(|m| !m.is_dependency);
	}

	if unrequired {
		mods.retain(|m| modpack.dependents_of(&m.project_id).len() == 0);
	}

	mods
}

pub fn filter_mods_by_options(modpack: &Modpack, parsed_args: &ParsedArgs) -> Vec<LocalMod> {
	filter_mods(
		modpack,
		&parsed_args.args,
		parsed_args.option(EXPLICIT),
		parsed_args.option(DEPENDENCY),
		parsed_args.option(UNREQUIRED),
	)
}
