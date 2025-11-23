use crate::requests;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RemoteMod {
	pub slug: String,
	title: Option<String>,
	description: Option<String>,
	pub game_versions: Option<Vec<String>>,
	pub loaders: Option<Vec<String>>,
	client_side: Option<String>,
	server_side: Option<String>,
}

impl RemoteMod {
	pub fn fetch(slug: &str) -> Result<RemoteMod, String> {
		let url = format!("https://api.modrinth.com/v2/project/{}", slug);
		requests::sync_get(&url, Vec::new())
			.inspect_err(|err_msg| {
				eprintln!("error: failed to fetch info about \'{}\'", slug);
				eprintln!("{}", err_msg);
			})
	}

	pub fn batch_fetch(slugs: &Vec<String>) -> BatchFetch {
		BatchFetch {
			n_slugs: slugs.len() as u8,
			slugs_iter: slugs.clone().into_iter(),
			n_failed: 0,
		}
	}

	pub fn get_title(&self) -> String {
		self.title
			.clone()
			.unwrap_or("<No title>".to_string())
	}

	pub fn get_description(&self) -> String {
		self.description
			.clone()
			.unwrap_or("<No description>".to_string())
	}

	pub fn get_client_side(&self) -> String {
		self.client_side
			.clone()
			.unwrap_or("<Not specified>".to_string())
	}

	pub fn get_server_side(&self) -> String {
		self.server_side
			.clone()
			.unwrap_or("<Not specified>".to_string())
	}

	pub fn format_game_versions<const N: usize>(&self) -> Option<String> {
		self.game_versions
			.clone()
			.map(|game_versions| {
				let (chunks, remainder) = game_versions.as_chunks::<N>();

				let mut formatted = String::new();

				for chunk in chunks {
					for version in chunk {
						formatted.push_str(version);
						formatted.push_str("  ");
					}
					formatted.push('\n');
				}
				for version in remainder {
					formatted.push_str(version);
					formatted.push_str("  ");
				}
				formatted.push('\n');

				formatted
			})
	}

	pub fn format_loaders(&self) -> Option<String> {
		self.loaders.clone().map(|loaders| loaders.join("  "))
	}
}

pub struct BatchFetch {
	slugs_iter: std::vec::IntoIter<String>,
	n_failed: u8,
	n_slugs: u8,
}

impl Iterator for BatchFetch {
	type Item = (String, Result<RemoteMod, String>);

	fn next(&mut self) -> Option<Self::Item> {
		match self.slugs_iter.next() {
			Some(slug) => {
				let remote_mod = RemoteMod::fetch(&slug);

				if remote_mod.is_err() {
					self.n_failed += 1;
				}

				Some((slug, remote_mod))
			},
			// after iterating over all mods
			None => {
				if self.n_failed > 0 {
					eprintln!("warning: {} request(s) failed", self.n_failed);
				}

				if self.n_failed == self.n_slugs {
					eprintln!("warning: all requests failed");
				}

				None
			}
		}
	}
}
