use crate::arg_parser::ParsedArgs;
use super::dispatcher;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		_remove_command_dummy,
		&[]
	);
}

fn _remove_command_dummy<T>(_: T) {
	todo!("remove operation");
}
