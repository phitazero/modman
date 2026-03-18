use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	#[command(short_flag = 'S')]
	Sync(SyncArgs),

	#[command(short_flag = 'Q')]
	Query(QueryArgs),

	#[command(short_flag = 'R')]
	Remove(RemoveArgs),

	#[command(short_flag = 'M')]
	Modpack(ModpackArgs),
}

#[derive(Args, Debug)]
pub struct SyncArgs {
	// -S

	/// Don't install dependencies
	#[arg(short = 'j', long)]
	#[arg(conflicts_with_all = ["upgrade", "info", "search"])]
	pub no_deps: bool,


	// -Su
	/// Reinstall/upgrade mods
	#[arg(short, long)]
	#[arg(conflicts_with_all = ["info", "search"])]
	pub upgrade: bool,


	// -Si

	/// Fetch detailed info about a mod
	#[arg(short, long)]
	#[arg(conflicts_with_all = ["upgrade", "search"])]
	pub info: bool,

	/// Fetch dependencies
	#[arg(short, long, requires = "info")]
	#[arg(conflicts_with_all = ["upgrade", "search"])]
	pub fetch_deps: bool,


	// -Ss

	/// Search for mods by a query
	#[arg(short, long)]
	#[arg(conflicts_with_all = ["upgrade", "info"])]
	pub search: bool,

	/// Show all fetched search results
	#[arg(short, long, requires = "search")]
	#[arg(conflicts_with_all = ["upgrade", "info"])]
	pub all: bool,

	/// Don't filter to be compatible with current modpack
	#[arg(short = 'd', long, requires = "search")]
	#[arg(conflicts_with_all = ["upgrade", "info"])]
	pub no_filter: bool,


	// Args

	#[arg(required_unless_present = "upgrade")]
	pub args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct QueryArgs {
	// For all:

	#[command(flatten)]
	pub filters: Filters,


	// -Q

	/// Don't show version numbers
	#[arg(short, long)]
	#[arg(conflicts_with_all = ["info"])]
	pub quiet: bool,


	// -Qi

	#[arg(short, long)]
	pub info: bool,

	/// Fetch the slugs for missing dependencies
	#[arg(short = 'f', long = "fetch")]
	#[arg(requires = "info")]
	pub fetch_missing_slugs: bool,


	// Args

	pub args: Vec<String>,
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

#[derive(Args, Debug)]
pub struct RemoveArgs {
	/// Also remove dependencies
	#[arg(short = 's', long = "deps")]
	pub remove_dependencies: bool,

	// Args

	#[arg(required = true)]
	pub args: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ModpackArgs {
	// -M

	/// Mod loader
	#[arg(short, long)]
	#[arg(conflicts_with_all = ["info"])]
	#[arg(required_unless_present = "info")]
	pub loader: Option<String>,

	/// Game version
	#[arg(short, long)]
	#[arg(conflicts_with_all = ["info"])]
	#[arg(required_unless_present = "info")]
	pub version: Option<String>,


	// -Mi

	/// Print info about the current modpack
	#[arg(short, long)]
	pub info: bool,
}
