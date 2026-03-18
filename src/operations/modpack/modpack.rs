use crate::utils;
use crate::config;
use crate::Modpack;
use crate::cli::modpack::ModpackInitArgs;
use std::process::exit;
use std::path::PathBuf;

const LOADERS: [&str; 4] = [
	"forge",
	"neoforge",
	"fabric",
	"quilt",
];

pub fn command_modpack(args: ModpackInitArgs) {
	let ModpackInitArgs { loader, version } = args;

	if !LOADERS.contains(&loader.as_str()) {
		eprintln!("fatal: loader \'{loader}\' not recognized");
		eprintln!("Supported loaders: {}", LOADERS.join(", "));
		exit(1);
	}

	let minecraft_dir_path = utils::current_dir()
		.join("minecraft");

	let result = std::fs::remove_dir_all(&minecraft_dir_path);

	if result.is_err() 
		&& result
			.err()
			.unwrap()
			.kind() != std::io::ErrorKind::NotFound
	{
		eprintln!("fatal: failed to remove 'minecraft' directory");
		exit(1);
	}
	let result = std::fs::create_dir(&minecraft_dir_path);

	if result.is_err() {
		eprintln!("fatal: failed to create 'minecraft' directory");
		exit(1);
	}

	let result = std::fs::create_dir(minecraft_dir_path.join("mods"));

	if result.is_err() {
		eprintln!("fatal: failed to create mods directory");
		exit(1);
	}

	create_symlinks(
		config().get_dot_minecraft_path(),
		&minecraft_dir_path,
		&[
			"resourcepacks",
			"shaderpacks",
			"saves",
			"screenshots",
			"hotbar.nbt",
			"options.txt",
			"servers.dat",
		],
	);

	Modpack {
		loader: loader.to_string(),
		version: version.to_string(),
		mods: Vec::new(),
	}.save();
}

fn create_symlinks(
	dot_minecraft_path: PathBuf,
	minecraft_dir_path: &PathBuf,
	items: &[&str],
) {
	for item in items {
		let result = std::os::unix::fs::symlink(
			dot_minecraft_path.join(item),
			minecraft_dir_path.join(item)
		);

		if result.is_err() {
			eprintln!("fatal: couldn't symlink \'{item}\'");
			exit(1);
		}
	}
}
