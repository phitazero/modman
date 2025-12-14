use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LocalMod {
	slug: String,
	project_id: String,
	dependencies: Vec<String>,
	file: String,
	is_dependency: bool,
}
