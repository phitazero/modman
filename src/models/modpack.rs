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
					eprintln!("fatal: failed to save modpack manifest");
					exit(1);
				}
			},
			Err(_) => {
				eprintln!("fatal: failed to open modpack manifest");
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
						eprintln!("fatal: couldn't read manifest JSON");
						exit(1);
					}
				}
			},
			Err(err) => {
				if err.kind() == std::io::ErrorKind::NotFound {
					None
				} else {
					eprintln!("fatal: failed to open modpack manifest");
					exit(1);
				}
			}
		}
	}

	pub fn require_current() -> Modpack {
		Modpack::current().unwrap_or_else(|| {
			eprintln!("fatal: modpack required");
			exit(1);
		})
	}

	pub fn install_single(&mut self, version: Version, is_dep: bool) -> Result<(), String> {
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
			slug: version.slug,
			project_id: version.project_id,
			dependencies: deps,
			is_dependency: is_dep,
			file: version_file.filename.clone(),
			version_number: version.version_number,
			title: version.title,
		};

		self.mods.push(local_mod);
		self.save();

		eprintln!("Install single: done");

		Ok(())
	}

	pub fn install(&mut self, version: Version) -> Result<(), String> {
		eprintln!("Installing \'{}\'", version.slug);

		self.mods
			.iter_mut()
			.find(|m| m.project_id == version.project_id && m.is_dependency)
			.map(|m| {
				eprintln!("Mod \'{}\' found as dependency, promoting to explicitly installed", m.slug);
				m.is_dependency = false;
			});

		let installed: Vec<String> = self.mods
			.iter()
			.map(|m| m.project_id.clone())
			.collect();

		let mut deps_to_install = version.dependencies.clone();
		deps_to_install.retain(|m| {
			if installed.contains(m) {
				eprintln!("Dependency \'{m}\' already installed");
				false
			} else {
				true
			}
		});

		let mut versions_to_install: Vec<Version> = Vec::new();

		for dependency_id in deps_to_install.iter() {
			let dep_version = Version::fetch_latest(dependency_id, self)?;
			versions_to_install.push(dep_version);
		}

		// required after version is moved
		let version_slug = version.slug.clone();

		if !installed.contains(&version.project_id) {
			versions_to_install.push(version);
		} else {
			eprintln!("Mod \'{}\' already installed", version_slug);
		}

		for version_to_install in versions_to_install {
			let is_dependency = version_to_install.slug != version_slug;
			self.install_single(version_to_install, is_dependency)?;
		}

		self.save();

		eprintln!("Install: done");

		Ok(())
	}

	pub fn remove(&mut self, slug_or_id: &str) -> Result<(), String> {
		eprintln!("Removing \'{slug_or_id}\'");

		let mod_index = self.mods
			.iter()
			.position(|m| m.slug == slug_or_id || m.project_id == slug_or_id)
			.ok_or_else(|| format!("mod \'{slug_or_id}\' not found"))?;

		let local_mod = self.mods.swap_remove(mod_index);
		self.save();

		let filename = &local_mod.file;

		let file_path = utils::current_dir()
			.join("minecraft")
			.join("mods")
			.join(filename);

		eprintln!("Removing mod file: \'{filename}\'");

		if !file_path.exists() {
			return Err(format!("file \'{filename}\' doesn't exist in mods directory"));
		}

		std::fs::remove_file(file_path)
			.map_err(|err| format!("couldn't remove file \'{filename}\': {err}"))?;

		eprintln!("Remove: done");
		Ok(())
	}
}
