use crate::arg_parser::ParsedArgs;
use super::dispatcher;

mod filter;

mod query;
mod info;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		query::command_query,
		&[
			('i', info::command_info)
		]
	);
}
