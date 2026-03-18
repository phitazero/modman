use crate::cli::sync::*;

mod info;
mod search;
mod sync;
mod upgrade;

pub fn dispatch(args: SyncArgs) {
	let auto_install_deps = !args.no_deps;

	match args.subcommand {
		SyncCommand::Install(args) =>
			sync::command_sync(args, !auto_install_deps),

		SyncCommand::Upgrade(args) => upgrade::command_upgrade(args),
		SyncCommand::Info(args) => info::command_info(args),
		SyncCommand::Search(args) => search::command_search(args),
	}
}
