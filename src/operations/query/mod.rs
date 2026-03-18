use crate::cli::QueryArgs;

mod query;
mod info;

pub fn dispatch(args: QueryArgs) {
	if args.info {
		info::command_info(args);
	} else {
		query::command_query(args);
	}
}
