use crate::arg_parser::ParsedArgs;
use super::dispatcher;

mod remove;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		remove::command_remove,
		&[]
	);
}
