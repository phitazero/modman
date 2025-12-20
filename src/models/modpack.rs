use serde::{Serialize, Deserialize};
use std::fs::File;
use std::process::exit;
use crate::{utils, requests};
use crate::MANIFEST_FILENAME;
use crate::{LocalMod, Version, VersionFile};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Modpack {
	pub loader: String,
	pub version: String,
	pub mods: Vec<LocalMod>,
}

impl Modpack {
	pub fn save(&self) {
		let path = utils::current_dir()
			.join(MANIFEST_FILENAME);

		match File::create(path) {
			Ok(file) => {
				let result = serde_json::to_writer_pretty(&file, self);

				if result.is_err() {
					eprintln!("error: failed to save modpack manifest");
					exit(1);
				}
			},
			Err(_) => {
				eprintln!("error: failed to open modpack manifest");
				exit(1);
			}
		}
	}

	pub fn current() -> Option<Modpack> {
		let path = utils::current_dir()
			.join(MANIFEST_FILENAME);

		match File::open(path) {
			Ok(file) => {
				match serde_json::from_reader::<File, Modpack>(file) {
					Ok(modpack) => Some(modpack),
					Err(_) => {
						// not clarifying the error here because the manifest is not to edit manually
						eprintln!("error: couldn't read manifest JSON");
						exit(1);
					}
				}
			},
			Err(err) => {
				if err.kind() == std::io::ErrorKind::NotFound {
					None
				} else {
					eprintln!("error: failed to open modpack manifest");
					exit(1);
				}
			}
		}
	}

	pub fn require_current() -> Modpack {
		Modpack::current().unwrap_or_else(|| {
			eprintln!("error: modpack required");
			exit(1);
		})
	}

	pub fn install_single(&mut self, version: &Version, is_dep: bool) -> Result<(), String> {
		eprintln!(
			"Installing single \'{}\' : \'{}\'  (dependency={is_dep})",
			version.slug,
			version.version_number
		);

		let version_file = VersionFile::select_primary(&version.files)?;

		let target_path = utils::current_dir()
			.join("minecraft")
			.join("mods")
			.join(&version_file.filename);

		requests::sync_download(target_path, &version_file.url)?;

		let deps = if !is_dep {
			version.dependencies.clone()
		} else {
			Vec::new()
		};

		let local_mod = LocalMod {
			slug: version.slug.clone(),
			project_id: version.project_id.clone(),
			dependencies: deps,
			is_dependency: is_dep,
			file: version_file.filename.clone(),
			version_number: version.version_number.clone(),
		};

		self.mods.push(local_mod);
		self.save();

		Ok(())
	}

	pub fn install(&mut self, version: &Version) -> Result<(), String> {
		eprintln!("Installing \'{}\'", version.slug);

		self.mods
			.iter_mut()
			.find(|m| m.project_id == version.project_id)
			.map(|m| {
				eprintln!("Mod \'{}\' found as dependency, promoting to explicitly installed", m.slug);
				m.is_dependency = false;
			});

		let installed: Vec<String> = self.mods
			.iter()
			.map(|m| m.project_id.clone())
			.collect();

		let mut to_install = version.dependencies.clone();
		to_install.push(version.project_id.clone());
		to_install.retain(|m| !installed.contains(m));

		let mut versions_to_install: Vec<Version> = Vec::new();

		for (_, version) in Version::batch_fetch_latest(to_install, self) {
			versions_to_install.push(version?)
		}

		for version_to_install in versions_to_install.iter() {
			let is_dependency = version_to_install.slug != version.slug;
			self.install_single(version_to_install, is_dependency)?;
		}

		self.save();
		Ok(())
	}
}
