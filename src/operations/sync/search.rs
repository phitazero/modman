use crate::requests;
use crate::arg_parser::ParsedArgs;
use crate::modpack::Modpack;
use crate::config;
use std::process::exit;

pub fn command_search(parsed_args: &mut ParsedArgs) {
	parsed_args.check_validity(&['S', 's', 'a']);

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
	let limit: u8 = match parsed_args.option('a') {
		false => config().get_mods_search_limit(),
		true => 100, // max allowed by the API
	};

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

	let data: serde_json::Value = requests::sync_get(&url, params)?;

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
