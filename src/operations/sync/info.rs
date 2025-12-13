use crate::{RemoteMod, Modpack};
use crate::arg_parser::ParsedArgs;
use std::process::exit;

const PRINT_VERSION_CHUNKS: usize = 5;

pub fn command_info(parsed_args: ParsedArgs) {
	parsed_args.check_validity(&['S', 'i']);

	let args = &parsed_args.args;

	if args.len() == 0 {
		eprintln!("error: no target specified");
		exit(1);
	}

	let modpack = Modpack::current();

	for (slug, remote_mod_res) in RemoteMod::batch_fetch(args) {
		match remote_mod_res {
			Ok(remote_mod) => print_info(remote_mod, modpack.as_ref()),
			Err(_) => println!("Skipping \'{slug}\'\n"),
		}
	}
}

fn print_info(remote_mod: RemoteMod, modpack: Option<&Modpack>) {
	println!("\n");

	let title = remote_mod.get_title();
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
		print!("\n");

		let does_version_match = match remote_mod.game_versions {
			Some(versions) => versions.contains(&modpack.version),
			None => {
				eprintln!("warning: supported mod versions not specified");
				true
			},
		};

		let does_loader_match = match remote_mod.loaders {
			Some(loaders) => loaders.contains(&modpack.loader),
			None => {
				eprintln!("warning: supported mod loaders not specified");
				true
			},
		};

		if does_version_match && does_loader_match {
			println!("[+] This mod is supported");
			return;
		}

		if !does_loader_match {
			println!("[-] This mod doesn't support this modpack's loader");
		}

		if !does_version_match {
			println!("[-] This mod doesn't support this modpack's game version");
		}
	}
}
