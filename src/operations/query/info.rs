use super::filter::{filter_mods, EXPLICIT, DEPENDENCY, UNREQUIRED};
use crate::{Modpack, LocalMod, RemoteMod};
use crate::arg_parser::ParsedArgs;

pub fn command_info(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['Q', 'i', 'f', EXPLICIT, DEPENDENCY, UNREQUIRED]);

	let modpack = Modpack::require_current();
	let mods = 	filter_mods(&modpack, &parsed_args);

	for local_mod in mods {
		print_info(
			&local_mod,
			&modpack,
			parsed_args.option('f'),
		);
	}
}

pub fn print_info(
	local_mod: &LocalMod,
	modpack: &Modpack,
	fetch_missing_deps_slug: bool,
) {
	println!("========== {} ({}) ==========", local_mod.title, local_mod.slug);
	println!("({})",
		if local_mod.is_dependency { "dependency" } 
		else { "explicitly installed" }
	);

	println!("Version: {}", local_mod.version_number);
	println!("File: {}", local_mod.file);

	println!("Project ID: {}", local_mod.project_id);

	println!("Dependencies ({}): ", local_mod.dependencies.len());

	for dep_project_id in local_mod.dependencies.iter() {
		print!("  ");

		let dep_slug_opt = modpack.mods
			.iter()
			.find(|m| m.project_id == *dep_project_id)
			.map(|m| &m.slug);

		if let Some(dep_slug) = dep_slug_opt {
			println!("{dep_slug}");
		} else if fetch_missing_deps_slug {
			print!("Fetching missing dependency slug...");

			{
				use std::io::{stdout, Write};
				let _ = stdout().flush();
			}

			let result = RemoteMod::fetch_slug_and_title(&dep_project_id);

			// erase the line, move to its beginning (with the tab)
			print!("\x1b[2K");
			print!("\x1b[3G");

			match result {
				Ok((dep_slug, _)) => println!("{dep_slug} (missing)"),
				Err(_) => println!("{dep_project_id} (missing, couldn't fetch slug)"),
			}
		} else {
			println!("{dep_project_id} (missing)");
		}
	}

	let dependents = modpack.dependents_of(&local_mod.project_id);

	println!("Dependents ({}):", dependents.len());
	println!("  {}", dependents.join("\n  "));

	print!("\n\n");
}
