use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize, Clone)]
pub struct LocalMod {
	pub slug: String,
	pub title: String,
	pub project_id: String,
	pub dependencies: Vec<String>,
	pub file: String,
	pub is_dependency: bool,
	pub version_number: String,
}
