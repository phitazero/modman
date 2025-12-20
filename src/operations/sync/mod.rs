use crate::arg_parser::ParsedArgs;
use super::dispatcher;

mod info;
mod search;
mod sync;

pub fn dispatch(parsed_args: ParsedArgs) {
	dispatcher::dispatch(
		parsed_args,
		sync::command_sync,
		&[
			('i', info::command_info),
			('s', search::command_search)
		]
	)
}

fn _command_sync_dummy(_: ParsedArgs) { todo!("sync command"); }
