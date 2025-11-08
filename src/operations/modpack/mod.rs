use crate::arg_parser::ParsedArgs;
use super::dispatcher;

mod modpack;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		modpack::command_modpack,
		&[]
	);
}
