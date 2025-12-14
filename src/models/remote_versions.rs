use crate::Modpack;
use crate::requests;
use serde::{Deserialize, Deserializer};
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

#[derive(Deserialize)]
pub struct Version {
	version_number: String,
	project_id: String,
	#[serde(deserialize_with = "deserialize_dependencies")]
	dependencies: Vec<String>,
	files: Vec<VersionFile>,
}

fn deserialize_dependencies<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where D: Deserializer<'de> {
	#[derive(Deserialize)]
	struct Dependency {
		project_id: String,
	}

	Ok(
		Vec::<Dependency>::deserialize(deserializer)?
			.into_iter()
			.map(|d| d.project_id)
			.collect()	
	)
}

#[derive(Deserialize)]
struct VersionFile {
	filename: String,
	url: String,
	primary: bool,
}
