use serde::{Deserialize, Deserializer};
use crate::{Modpack, RemoteMod};
use crate::requests;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct Version {
	#[serde(default)]
	pub slug: String,
	pub version_number: String,
	pub project_id: String,
	#[serde(deserialize_with = "deserialize_dependencies")]
	pub dependencies: Vec<String>,
	pub files: Vec<VersionFile>,
}

impl Version {
	fn fetch_all_no_slugs(slug_or_id: &str, modpack: &Modpack) -> Result<Vec<Version>, String> {
		let url = format!("https://api.modrinth.com/v2/project/{slug_or_id}/version");

		let mut params: Vec<(&str, &str)> = Vec::new();

		let loaders_str = format!("[\"{}\"]", modpack.loader);
		params.push(("loaders", &loaders_str));

		let versions_str = format!("[\"{}\"]", modpack.version);
		params.push(("game_versions", &versions_str));

		let version_list: Vec<Version> = requests::sync_get(&url, params)?;

		Ok(version_list)
	}

	pub fn fetch_n_available(slug_or_id: &str, modpack: &Modpack) -> Result<usize, String> {
		Ok(Self::fetch_all_no_slugs(slug_or_id, modpack)?.len())
	}

	pub fn fetch_latest(slug_or_id: &str, modpack: &Modpack) -> Result<Version, String> {
		eprintln!("Fetching latest version for \'{slug_or_id}\'");

		let version_list: Vec<Version> = Self::fetch_all_no_slugs(slug_or_id, modpack)?;

		let mut latest = version_list
			.into_iter()
			.next()
			.ok_or(String::from("no versions present"))?;

		let slug = RemoteMod::fetch_slug(slug_or_id)?;
		latest.slug = slug;

		Ok(latest)
	}

	pub fn batch_fetch_latest(slugs: Vec<String>, modpack: &Modpack) -> BatchVersionFetch<'_> {
		BatchVersionFetch {
			n_slugs: slugs.len() as u8,
			slugs_iter: slugs.into_iter(),
			failed: Vec::new(),
			modpack
		}
	}
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

#[derive(Debug)]
#[derive(Deserialize)]
pub struct VersionFile {
	pub filename: String,
	pub url: String,
	primary: bool,
}

impl VersionFile {
	pub fn select_primary(files: &[VersionFile]) -> Result<&VersionFile, String> {
		let primary_opt = files
			.iter()
			.find(|f| f.primary);

		if let Some(primary) = primary_opt {
			return Ok(primary);
		}

		files.first()
			.ok_or(String::from("no files in version"))
	}
}

pub struct BatchVersionFetch<'a> {
	pub failed: Vec<String>,
	modpack: &'a Modpack,
	slugs_iter: std::vec::IntoIter<String>,
	n_slugs: u8,
}

impl Iterator for BatchVersionFetch<'_> {
	type Item = (String, Result<Version, String>);

	fn next(&mut self) -> Option<Self::Item> {
		match self.slugs_iter.next() {
			Some(slug) => {
				let version = Version::fetch_latest(&slug, self.modpack);

				if version.is_err() {
					self.failed.push(slug.clone());
				}

				Some((slug, version))
			},
			// after iterating over all slugs
			None => {
				let n_failed = self.failed.len() as u8;

				if n_failed > 0 {
					eprintln!("warning: {n_failed} request(s) failed for slugs/IDs:");
					eprintln!("{}", self.failed.join(", "));

					if n_failed == self.n_slugs {
						eprintln!("warning: all requests failed");
					}
				}

				None
			}
		}
	}
}
