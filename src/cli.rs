use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	#[command(short_flag = 'S')]
	Sync(sync::SyncArgs),

	#[command(short_flag = 'Q')]
	Query(query::QueryArgs),

	#[command(short_flag = 'R')]
	Remove(remove::RemoveArgs),

	#[command(short_flag = 'M')]
	Modpack(modpack::ModpackArgs),
}

pub mod sync {
	use clap::{Subcommand, Args};

	#[derive(Args, Debug)]
	pub struct SyncArgs {
		#[command(subcommand)]
		pub subcommand: SyncCommand,

		/// Don't install dependencies
		#[arg(short = 'j', long)]
		pub no_deps: bool,
	}

	#[derive(Subcommand, Debug)]
	pub enum SyncCommand {
		/// Install (add) mods
		#[command(short_flag = 'a')]
		Install(SyncInstallArgs),

		/// Reinstall/upgrade mods
		#[command(short_flag = 'u')]
		Upgrade(SyncUpgradeArgs),

		/// Fetch detailed info about a mod
		#[command(short_flag = 'i')]
		Info(SyncInfoArgs),

		/// Search for mods by a query
		#[command(short_flag = 's')]
		Search(SyncSearchArgs),
	}

	#[derive(Args, Debug)]
	pub struct SyncInstallArgs {
		/// Slugs of the mods to install
		#[arg(required = true)]
		pub slugs: Vec<String>,
	}

	#[derive(Args, Debug)]
	pub struct SyncSearchArgs {
		/// Show all fetched search results
		#[arg(short, long)]
		pub all: bool,

		/// Don't filter to be compatible with current modpack
		#[arg(short = 'd', long)]
		pub no_filter: bool,

		/// Search query
		pub query: String,
	}

	#[derive(Args, Debug)]
	pub struct SyncInfoArgs {
		/// Slugs of the mod to print info about
		pub slugs: Vec<String>,

		/// Fetch dependencies
		#[arg(short = 'd', long)]
		pub fetch_deps: bool,
	}

	#[derive(Args, Debug)]
	pub struct SyncUpgradeArgs {
		/// Slugs of the mods to upgrade. Leave empty to affect all
		pub slugs: Vec<String>,
	}
}

pub mod query {
	use clap::{Subcommand, Args};

	#[derive(Args, Debug)]
	pub struct QueryArgs {
		#[command(subcommand)]
		pub subcommand: QueryCommand,

		#[command(flatten)]
		pub filters: Filters,
	}
		
	#[derive(Args, Debug)]
	pub struct Filters {
		/// List mods installed as dependencies [filter]
		#[arg(short, long)]
		pub dependencies: bool,

		/// List mods installed explicitly [filter]
		#[arg(short, long)]
		pub explicit: bool,

		/// List mods not required by any other mod [filter]
		#[arg(short = 't', long)]
		pub unrequired: bool,
	}

	#[derive(Subcommand, Debug)]
	pub enum QueryCommand {
		/// List mods
		#[command(short_flag = 'l')]
		List(QueryListArgs),

		/// Print detailed info about mods
		#[command(short_flag = 'i')]
		Info(QueryInfoArgs),
	}

	#[derive(Args, Debug)]
	pub struct QueryListArgs {
		/// Slugs of the mods to check. Leave empty to list all
		pub slugs: Vec<String>,

		/// Don't show version numbers
		#[arg(short, long)]
		pub quiet: bool,
	}

	#[derive(Args, Debug)]
	pub struct QueryInfoArgs {
		/// Slugs of the mods to print info about. Leave empty to affect all
		pub slugs: Vec<String>,

		/// Fetch the slugs for missing dependencies
		#[arg(short = 'f', long = "fetch")]
		pub fetch_missing_slugs: bool,
	}
}

pub mod remove {
	use clap::Args;

	#[derive(Args, Debug)]
	pub struct RemoveArgs {
		/// Slugs of the mods to remove
		#[arg(required = true)]
		pub slugs: Vec<String>,

		#[arg(short = 's', long = "deps")]
		pub remove_dependencies: bool,
	}
}

pub mod modpack {
	use clap::{Subcommand, Args};

	#[derive(Args, Debug)]
	pub struct ModpackArgs {
		#[command(subcommand)]
		pub subcommand: ModpackCommand,
	}

	#[derive(Subcommand, Debug)]
	pub enum ModpackCommand {
		/// Initialize a modpack
		#[command(short_flag = 'c')]
		Init(ModpackInitArgs),

		/// Print info about the current modpack
		#[command(short_flag = 'i')]
		Info,
	}

	#[derive(Args, Debug)]
	pub struct ModpackInitArgs {
		/// Mod loader
		#[arg(short, long)]
		pub loader: String,

		/// Game version
		#[arg(short, long)]
		pub version: String,
	}
}
