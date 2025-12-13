use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Version {
	version_number: String,
	project_id: String,
	#[serde(deserialize_with = "deserialize_dependencies")]
	dependencies: Vec<String>,
	// file: VersionFile,
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
