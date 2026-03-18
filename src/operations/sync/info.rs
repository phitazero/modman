use crate::{RemoteMod, Modpack, Version, ResultTracker};
use crate::cli::sync::SyncInfoArgs;

const PRINT_VERSION_CHUNKS: usize = 5;

pub fn command_info(args: SyncInfoArgs) {
	let modpack = Modpack::current();

	let mut result_tracker = ResultTracker::new();

	for slug in &args.slugs {
		let result = print_info(
			slug,
			modpack.as_ref(),
			args.fetch_deps,
		);

		result_tracker.register(slug, &result);

		if let Err(err) = result {
			println!("An error occured while getting info about \'{slug}\'");
			eprintln!("error: {err}");
		}

		print!("\n\n");
	}

	result_tracker.summarize();
}

fn print_info(
	slug: &str,
	modpack: Option<&Modpack>,
	fetch_deps: bool
) -> Result<(), String> {
	let remote_mod = RemoteMod::fetch(slug)?;

	let title = &remote_mod.title;
	println!("========== {title} ({}) ==========", remote_mod.slug);

	println!("{}", remote_mod.get_description());

	match remote_mod.format_game_versions::<PRINT_VERSION_CHUNKS>() {
		Some(formatted) => println!("Supported game versions:\n{formatted}"),
		None => println!("<Game versions not specified>"),
	}

	match remote_mod.format_loaders() {
		Some(formatted) => println!("Supported mod loaders:\n  {formatted}"),
		None => println!("<Supported mod loaders not specified>"),
	}

	println!("Client side: {}", remote_mod.get_client_side());
	println!("Server side: {}", remote_mod.get_server_side());

	if let Some(modpack) = modpack {
		println!("Compatibility with current modpack");

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

		println!("  Game version supported:  [{}]", game_version_criterion.as_symbol());
		println!("  Loader supported:        [{}]", loader_criterion.as_symbol());
		println!("  Any versions found:      [{}]", versions_available_criterion.as_symbol());
		println!("  IS THIS MOD COMPATIBLE:  [{}]", verdict.as_symbol());
	}

	if fetch_deps {
		let modpack = Modpack::require_current();

		let latest_version = Version::fetch_latest(slug, &modpack)?;

		// erase the fetch log message after it's finished
		print!("\x1b[A"); // move cursor up 1 line
		print!("\x1b[2K"); // erase the line

		let dependency_ids = latest_version.dependencies;

		println!("Dependencies ({}):", dependency_ids.len());

		for dependency in dependency_ids {
			println!("  {}",
				match RemoteMod::fetch_slug_and_title(&dependency) {
					Ok((slug, _)) => slug,
					Err(_) => format!("{dependency} (couldn't fetch slug)"),
				}
			);
		}
	}

	Ok(())
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
