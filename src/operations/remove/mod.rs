use crate::cli::remove::RemoveArgs;

mod remove;

pub fn dispatch(args: RemoveArgs) {
	remove::command_remove(args);
}
