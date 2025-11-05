use crate::RemoteMod;
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

	for (slug, remote_mod_res) in RemoteMod::batch_fetch(args) {
		match remote_mod_res {
			Ok(remote_mod) => print_info(remote_mod),
			Err(_) => println!("Skipping \'{slug}\'\n"),
		}
	}
}

fn print_info(remote_mod: RemoteMod) {
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
}
