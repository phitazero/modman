use std::path::PathBuf;
use std::process::exit;
use std::env;

pub fn assert_directory(path: &PathBuf) {
	if !path.exists() {
		eprintln!("error: directory path doesn't exist: {}", path.display());
		exit(1);
	}

	if !path.is_dir() {
		eprintln!("error: directory path is not a directory: {}", path.display());
		exit(1);
	}
}

pub fn current_dir() -> PathBuf {
	match env::current_dir() {
		Ok(dir) => dir,
		Err(_) => {
			eprintln!("error: can't access current directory");
			exit(1);
		}
	}
}
