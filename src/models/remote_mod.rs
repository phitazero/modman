use crate::requests;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RemoteMod {
	pub slug: String,
	pub title: String,
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
			.map_err(|err_msg| {
				format!("failed to fetch info about \'{slug}\'\n{err_msg}")
			})
	}

	pub fn fetch_slug_and_title(slug_or_id: &str) -> Result<(String, String), String> {
		let remote_mod = RemoteMod::fetch(slug_or_id)?;
		Ok((
			remote_mod.slug,
			remote_mod.title,
		))
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
			.map(|mut game_versions| {
				game_versions.reverse();

				let (chunks, remainder) = game_versions.as_chunks::<N>();

				let mut formatted = String::new();

				for chunk in chunks {
					for version in chunk {
						formatted.push_str("  ");
						formatted.push_str(version);
					}
					formatted.push('\n');
				}
				for version in remainder {
					formatted.push_str("  ");
					formatted.push_str(version);
				}

				formatted
			})
	}

	pub fn format_loaders(&self) -> Option<String> {
		self.loaders.clone().map(|loaders| loaders.join("  "))
	}
}
