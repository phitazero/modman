use crate::arg_parser::ParsedArgs;
use crate::{LocalMod, Modpack};

pub const EXPLICIT: char = 'e';
pub const DEPENDENCY: char = 'd';
pub const UNREQUIRED: char = 't';

pub fn filter_mods_by_options(modpack: &Modpack, parsed_args: &ParsedArgs) -> Vec<LocalMod> {
	modpack.filter_mods(
		&parsed_args.args,
		parsed_args.option(EXPLICIT),
		parsed_args.option(DEPENDENCY),
		parsed_args.option(UNREQUIRED),
	)
}
