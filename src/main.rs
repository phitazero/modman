mod requests;
mod operations;
mod config;
mod models;
mod utils;
mod cli;

use config::config;
use models::remote_mod::RemoteMod;
use models::modpack::Modpack;
use models::version::{Version, VersionFile};
use models::local_mod::LocalMod;
use models::result_tracker::ResultTracker;
use cli::Command;
use clap::Parser;

const MANIFEST_FILENAME: &str = "modman-modpack.json";

fn main() {
	let cli = cli::Cli::parse();

	match cli.command {
		Command::Sync(args) => operations::sync::dispatch(args),
		Command::Query(args) => operations::query::dispatch(args),
		Command::Remove(args) => operations::remove::dispatch(args),
		Command::Modpack(args) => operations::modpack::dispatch(args),
	}
}
