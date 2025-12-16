use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LocalMod {
	pub slug: String,
	pub project_id: String,
	pub dependencies: Vec<String>,
	pub file: String,
	pub is_dependency: bool,
}
