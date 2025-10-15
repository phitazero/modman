use crate::requests;
use crate::arg_parser::ParsedArgs;

const PRINT_VERSION_CHUNKS: usize = 5;

pub fn dispatch(mut parsed_args: ParsedArgs) {
	if parsed_args.exhaust_option('h') {
		print_help();
	}
}

fn print_info(slug: String) -> Result<(), String> {
	println!("\n");

	let url = format!("https://api.modrinth.com/v2/project/{}", slug);
	let data = requests::sync_get(&url, Vec::new())?;

	// from now on all responsibility for .unwrap()'s is moved onto the API
	// if an .unwrap() panics - idk, not my fault

	let title = &data["title"];
	if !title.is_null() {
		print!("{} ", title.as_str().unwrap());
	}

	println!("({slug})\n");

	let desc = &data["description"];
	if !desc.is_null() {
		println!("{}\n", desc.as_str().unwrap());
	}

	let game_versions = &data["game_versions"];
	if !game_versions.is_null() {
		let game_versions = game_versions.as_array().unwrap();

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
		
	}

	let loaders = &data["loaders"];
	if !loaders.is_null() {
		let loaders = loaders.as_array().unwrap();

		println!("Supported mod loaders:");

		for loader in loaders {
			print!("{}  ", loader.as_str().unwrap());
		}
		println!("\n");
		
	}

	Ok(())
}

fn print_help() {
	todo!("help");
}
