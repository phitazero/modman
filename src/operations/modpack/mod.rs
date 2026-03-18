use crate::cli::ModpackArgs;

mod modpack;
mod info;

pub fn dispatch(args: ModpackArgs) {
	if args.info {
		info::command_info()
	} else {
		modpack::command_modpack(args);
	}
}
