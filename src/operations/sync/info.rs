use crate::requests;
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

	let url = format!("https://api.modrinth.com/v2/project/{}", slug);
	let data = requests::sync_get(&url, Vec::new())?;

	let title = match &data["title"] {
		serde_json::Value::String(title) => title,
		_ => &"<No title>".to_string(),
	};
	println!("[ {title} ({slug}) ]\n");

	let desc = match &data["description"] {
		serde_json::Value::String(desc) => desc,
		_ => &"<No description>".to_string(),
	};
	println!("{desc}\n");

	match &data["game_versions"] {
		serde_json::Value::Array(game_versions) => {
			let (chunks, remainder) = game_versions.as_chunks::<PRINT_VERSION_CHUNKS>();

			println!("Supported game versions:");

			for chunk in chunks {
				for version in chunk {
					print!("{}  ", version.as_str().unwrap());
				}
				println!();
			}
			for version in remainder {
				print!("{}  ", version.as_str().unwrap());
			}
			println!("\n");
		},
		_ => println!("<Game versions not specified>\n"),
	}

	match &data["loaders"] {
		serde_json::Value::Array(loaders) => {
			println!("Supported mod loaders:");

			for loader in loaders {
				print!("{}  ", loader.as_str().unwrap());
			}
			println!("\n");
		},
		_ => println!("<Supported mod loaders not specified>\n"),
	}

	Ok(())
}
