use serde::{Deserialize, Deserializer};

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
