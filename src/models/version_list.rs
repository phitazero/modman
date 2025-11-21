use crate::Modpack;
use crate::requests;
use serde::Deserialize;
use std::ops::Deref;
use std::process::exit;

// dummy
#[derive(Deserialize)]
pub struct Version {}

#[derive(Deserialize)]
pub struct VersionList(Vec<Version>);

impl Deref for VersionList {
	type Target = Vec<Version>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl VersionList {
	pub fn fetch(slug: &str, modpack: &Modpack) -> VersionList {
		let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);

		let mut params: Vec<(&str, &str)> = Vec::new();

		let loaders_str = format!("[\"{}\"]", modpack.loader);
		params.push(("loaders", &loaders_str));

		let versions_str = format!("[\"{}\"]", modpack.version);
		params.push(("game_versions", &versions_str));

		requests::sync_get(&url, params)
			.unwrap_or_else(|err_msg| {
				eprintln!("error: couldn't fetch versions for \'{slug}\'");
				eprintln!("{err_msg}");
				exit(1);
			})
	}
}
