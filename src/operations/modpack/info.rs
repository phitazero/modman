use crate::Modpack;
use crate::utils;

pub fn command_info() {
	let modpack = Modpack::require_current();

	/*
	Path.file_name() returns None in two cases:
	1) It's root directory.
	If code execution reached this point, Modpack::require_current()
	hasn't exited, which means that we're in a valid modpack, which can't
	be created in root dir. Alternatively it's possible with modpack
	manifest manually created in root dir, but i'm not an idiot to do that
	2) Path terminates in ..
	env::current_dir() can't return a path terminating in ..

	Hence .file_name() will always be a Some and .unwrap is safe.
	*/
	let path = utils::current_dir();
	let name = path.file_name().unwrap();

	println!("========== {} ==========", name.display());
	println!("Version: {}", modpack.version);
	println!("Loader: {}", modpack.loader);
}
