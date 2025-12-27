use crate::arg_parser::ParsedArgs;
use super::dispatcher;

mod modpack;
mod info;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		modpack::command_modpack,
		&[
			('i', info::command_info),
		]
	);
}
