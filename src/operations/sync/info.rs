use crate::{RemoteMod, Modpack, Version, ResultTracker};
use crate::arg_parser::ParsedArgs;
use std::process::exit;

const PRINT_VERSION_CHUNKS: usize = 5;

pub fn command_info(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S', 'i']);

	let args = parsed_args.args;

	if args.len() == 0 {
		eprintln!("fatal: no target specified");
		exit(1);
	}

	let modpack = Modpack::current();

	let mut result_tracker = ResultTracker::new();

	for slug in args.iter() {
		let remote_mod_res = RemoteMod::fetch(slug);

		result_tracker.register(slug, &remote_mod_res);

		match remote_mod_res {
			Ok(remote_mod) => print_info(remote_mod, modpack.as_ref()),
			Err(err_msg) => {
				eprintln!("error: {err_msg}");
				println!("Skipping \'{slug}\'\n");
			},
		}
	}

	result_tracker.summarize();
}

fn print_info(remote_mod: RemoteMod, modpack: Option<&Modpack>) {
	let title = &remote_mod.title;
	println!("[ {title} ({}) ]\n", remote_mod.slug);

	println!("{}\n", remote_mod.get_description());

	match remote_mod.format_game_versions::<PRINT_VERSION_CHUNKS>() {
		Some(formatted) => println!("Supported game versions:\n{formatted}"),
		None => println!("<Game versions not specified>"),
	}

	match remote_mod.format_loaders() {
		Some(formatted) => println!("Supported mod loaders:\n{formatted}"),
		None => println!("<Supported mod loaders not specified>"),
	}

	print!("\n");

	println!("Client side: {}", remote_mod.get_client_side());
	println!("Server side: {}", remote_mod.get_server_side());

	if let Some(modpack) = modpack {
		println!("\nChecking compatibility with current modpack");

		let game_version_criterion = match remote_mod.game_versions {
			Some(versions) => Criterion::from(versions.contains(&modpack.version)),
			None => Criterion::Unknown,
		};

		let loader_criterion = match remote_mod.loaders {
			Some(loaders) => Criterion::from(loaders.contains(&modpack.loader)),
			None => Criterion::Unknown,
		};

		let versions_available_criterion =
			if game_version_criterion.is_passing() && loader_criterion.is_passing() {
				match Version::fetch_n_available(&remote_mod.slug, modpack) {
					Ok(0) => Criterion::NotSupported,
					Ok(_) => Criterion::Supported,
					Err(_) => Criterion::Unknown,
				}
			} else {
				Criterion::NotSupported
			};

		let verdict = 
			game_version_criterion
			.and(loader_criterion)
			.and(versions_available_criterion);

		println!("Game version supported:  [{}]", game_version_criterion.as_symbol());
		println!("Loader supported:        [{}]", loader_criterion.as_symbol());
		println!("Any versions found:      [{}]", versions_available_criterion.as_symbol());
		println!("IS THIS MOD COMPATIBLE:  [{}]", verdict.as_symbol());
	}

	print!("\n\n");
}

#[derive(Clone, Copy)]
enum Criterion {
	NotSupported,
	Unknown,
	Supported,
}

impl Criterion {
	fn as_symbol(&self) -> char {
		match self {
			Criterion::NotSupported => '-',
			Criterion::Unknown => '?',
			Criterion::Supported => '+',
		}
	}

	fn is_passing(&self) -> bool {
		match self {
			Criterion::NotSupported => false,
			Criterion::Unknown | Criterion::Supported => true,
		}
	}

	fn and(self, other: Criterion) -> Criterion {
		match (self, other) {
			(Criterion::Supported, Criterion::Supported) => Criterion::Supported,

			(Criterion::Unknown, Criterion::Unknown)
			| (Criterion::Unknown, Criterion::Supported)
			| (Criterion::Supported, Criterion::Unknown) => Criterion::Unknown,

			_ => Criterion::NotSupported,
		}
	}
}

impl From<bool> for Criterion {
	fn from(value: bool) -> Self {
		match value {
			false => Criterion::NotSupported,
			true => Criterion::Supported,
		}
	}
}
