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

	pub fn batch_fetch(slugs: Vec<String>, modpack: &Modpack) -> BatchVersionListFetch<'_> {
		BatchVersionListFetch {
			n_slugs: slugs.len() as u8,
			slugs_iter: slugs.into_iter(),
			failed: Vec::new(),
			modpack
		}
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn latest(&self) -> Result<&Version, String> {
		self.0.last()
			.ok_or(String::from("no versions present"))
	}
}

pub struct BatchVersionListFetch<'a> {
	pub failed: Vec<String>,
	modpack: &'a Modpack,
	slugs_iter: std::vec::IntoIter<String>,
	n_slugs: u8,
}

impl Iterator for BatchVersionListFetch<'_> {
	type Item = (String, Result<VersionList, String>);

	fn next(&mut self) -> Option<Self::Item> {
		match self.slugs_iter.next() {
			Some(slug) => {
				let version_list = VersionList::fetch(&slug, self.modpack);

				if version_list.is_err() {
					self.failed.push(slug.clone());
				}

				Some((slug, version_list))
			},
			// after iterating over all slugs
			None => {
				let n_failed = self.failed.len() as u8;

				if n_failed > 0 {
					eprintln!("warning: {n_failed} request(s) failed for slugs:");
					eprintln!("{}", self.failed.join(", "));
				}

				if n_failed == self.n_slugs {
					eprintln!("warning: all requests failed");
				}

				None
			}
		}
	}
}
