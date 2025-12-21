use crate::arg_parser::ParsedArgs;
use super::dispatcher;

mod query;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		query::command_query,
		&[]
	);
}
