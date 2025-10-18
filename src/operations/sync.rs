use crate::requests;
use crate::arg_parser::ParsedArgs;
use std::process::exit;

const PRINT_VERSION_CHUNKS: usize = 5;

// dummy, will be implement propertly in the future
struct Modpack { loader: String, version: String }

pub fn dispatch(mut parsed_args: ParsedArgs) {
	if parsed_args.exhaust_option('h') {
		print_help();
	}

	if parsed_args.exhaust_option('i') {
		command_info(&mut parsed_args);
	}

	if parsed_args.exhaust_option('s') {
		command_search(&mut parsed_args);
	}
}

fn command_info(parsed_args: &mut ParsedArgs) {
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

fn command_search(parsed_args: &mut ParsedArgs) {
	let args = &parsed_args.args;

	if args.len() == 0 {
		eprintln!("error: no target specified");
		exit(1);
	}

	if args.len() > 1 {
		eprintln!("error: too many arguments");
		exit(1);
	}

	let slug = &args[0];

	let modpack: Option<Modpack> = None;
	// let modpack: Option<Modpack> = Some(Modpack{loader: "quilt".to_string(), version:"1.21.4".to_string()});
	let limit: u8 = /*get from config*/ 10;

	let result = search_by_slug(slug, modpack, limit);

	match result {
		Ok((n_hits, n_total_hits)) => {
			println!("Showing {} mods out of {} total", n_hits, n_total_hits);
			println!("Top finds are placed the lowest");
		},
		Err(err_msg) => {
			eprintln!("error: search failed");
			eprintln!("{}", err_msg);
		}
	}
}

fn search_by_slug(slug: &String, modpack: Option<Modpack>, limit: u8) -> Result<(u8, u8), String> {
	let url = format!("https://api.modrinth.com/v2/search");

	let mut params: Vec<(&str, &str)> = Vec::new();
	params.push(("query", slug.as_str()));

	let facets_str = construct_facets(&modpack);
	params.push(("facets", facets_str.as_str()));

	let limit_str = limit.to_string();
	params.push(("limit", limit_str.as_str()));

	println!("{:?}", params);

	let data = requests::sync_get(&url, params)?;

	let mut hits = data["hits"].as_array().unwrap().clone();

	// top finds are printed the lowest, so that i don't need to scroll up to see them
	hits.reverse();

	for hit in &hits {
		print!(
			"[ {}",
			match hit["title"].clone() {
				serde_json::Value::String(title) => title,
				_ => "<No title>".to_string(),
			}
		);

		println!(
			" ({}) ]",
			match hit["slug"].clone() {
				serde_json::Value::String(slug) => slug,
				_ => "<No slug>".to_string(),
			}
		);

		println!("{}\n\n",
			match hit["description"].clone() {
				serde_json::Value::String(desc) => desc,
				_ => "<No desc>".to_string(),
			}
		);
	}

	let n_hits = hits.len() as u8;
	let n_total_hits = data["total_hits"].as_i64().unwrap() as u8;

	Ok((n_hits, n_total_hits))
}

fn construct_facets(modpack: &Option<Modpack>) -> String {
	let mut facets: Vec<String> = Vec::new();
	facets.push("[\"project_type:mod\"]".to_string());
	
	match modpack {
		None => (),
		Some(modpack) => {
			facets.push(format!("[\"categories:{}\"]", modpack.loader));
			facets.push(format!("[\"versions:{}\"]", modpack.version));
		}
	}

	format!("[{}]", facets.join(","))
}


fn print_info(slug: &String) -> Result<(), String> {
	println!("\n");

	let url = format!("https://api.modrinth.com/v2/project/{}", slug);
	let data = requests::sync_get(&url, Vec::new())?;

	// from now on all responsibility for .unwrap()'s is moved onto the API
	// if an .unwrap() panics - idk, not my fault

	let title = &data["title"];
	print!(
		"[ {}",
		match title {
			serde_json::Value::String(title) => format!("{} ", title),
			_ => String::new(),
		}
	);

	println!("({slug}) ]\n");

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
