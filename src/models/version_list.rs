use serde::Deserialize;
use crate::{Modpack, Version};
use crate::requests;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct VersionList(Vec<Version>);

impl VersionList {
	pub fn fetch(slug: &str, modpack: &Modpack) -> Result<VersionList, String> {
		let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);

		let mut params: Vec<(&str, &str)> = Vec::new();

		let loaders_str = format!("[\"{}\"]", modpack.loader);
		params.push(("loaders", &loaders_str));

		let versions_str = format!("[\"{}\"]", modpack.version);
		params.push(("game_versions", &versions_str));

		let mut version_list: VersionList = requests::sync_get(&url, params)?;

		for version in version_list.0.iter_mut() {
			version.slug = String::from(slug);
		}

		Ok(version_list)
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn latest(&self) -> Result<&Version, String> {
		self.0.last()
			.ok_or(String::from("no versions present"))
	}
}
