use crate::{Modpack, Version};
use crate::requests;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Deserialize)]
pub struct VersionList(Vec<Version>);

impl Deref for VersionList {
	type Target = Vec<Version>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl VersionList {
	pub fn fetch(slug: &str, modpack: &Modpack) -> Result<VersionList, String> {
		let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);

		let mut params: Vec<(&str, &str)> = Vec::new();

		let loaders_str = format!("[\"{}\"]", modpack.loader);
		params.push(("loaders", &loaders_str));

		let versions_str = format!("[\"{}\"]", modpack.version);
		params.push(("game_versions", &versions_str));

		requests::sync_get(&url, params)
	}
}
