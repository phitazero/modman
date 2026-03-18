use crate::cli::modpack::{ModpackArgs, ModpackCommand};

mod modpack;
mod info;

pub fn dispatch(args: ModpackArgs) {
	match args.subcommand {
		ModpackCommand::Init(args) => modpack::command_modpack(args),
		ModpackCommand::Info => info::command_info(),
	}
}
