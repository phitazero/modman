use crate::Modpack;
use serde::Deserialize;
use std::ops::Deref;

// dummy
#[derive(Deserialize)]
pub struct Version;

#[derive(Deserialize)]
pub struct VersionList(Vec<Version>);

impl Deref for VersionList {
	type Target = Vec<Version>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
