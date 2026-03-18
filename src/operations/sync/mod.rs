use crate::cli::SyncArgs;

mod info;
mod search;
mod sync;
mod upgrade;

pub fn dispatch(args: SyncArgs) {
	if args.info {
		info::command_info(args);
	} else if args.search {
		search::command_search(args);
	} else if args.upgrade {
		upgrade::command_upgrade(args);
	} else {
		sync::command_sync(args);
	}
}
