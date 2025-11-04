use crate::requests;
use crate::RemoteMod;
use crate::arg_parser::ParsedArgs;
use std::process::exit;

const PRINT_VERSION_CHUNKS: usize = 5;

pub fn command_info(parsed_args: &mut ParsedArgs) {
	parsed_args.check_validity(&['S', 'i']);

	let args = &parsed_args.args;

	if args.len() == 0 {
		eprintln!("error: no target specified");
		exit(1);
	}

	let mut n_successful = 0;

	for arg in args {
		let result = print_info(&arg);

		match result {
			Ok(()) => { n_successful += 1; },
			Err(err_msg) => {
				eprintln!("error: failed to fetch info about \'{}\'", arg);
				eprintln!("{}", err_msg);
				println!("Skipped \'{}\'\n", arg)
			},
		}
	}

	if n_successful < args.len() {
		eprintln!("Skipped {} mod(s)", args.len() - n_successful);
	}

	if n_successful == 0 {
		eprintln!("All requests failed!");
	}
}

fn print_info(slug: &String) -> Result<(), String> {
	println!("\n");

	let remote_mod = RemoteMod::fetch(slug)?;

	let title = remote_mod.get_title();
	println!("[ {title} ({slug}) ]\n");

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

	Ok(())
}
