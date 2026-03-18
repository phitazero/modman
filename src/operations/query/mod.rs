use crate::cli::query::{QueryArgs, QueryCommand};

mod query;
mod info;

pub fn dispatch(args: QueryArgs) {
	let filters = args.filters;

	match args.subcommand {
		QueryCommand::List(args) => query::command_query(args, filters),
		QueryCommand::Info(args) => info::command_info(args, filters),
	}
}
