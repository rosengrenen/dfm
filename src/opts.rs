use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[clap(
	name = "dfm",
	version = env!("CARGO_PKG_VERSION"),
	author = "Rasmus Rosengren <rasmus.rosengren@protonmail.com>",
	about = "Utility to manage dotfiles"
)]
pub struct Opts {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	#[clap(about = "Apply changes")]
	Apply {
		#[clap(
			short,
			long,
			default_value = ".df",
			help = "Path to source repo, absolute path or path relative to $HOME"
		)]
		repo_path: PathBuf,

		#[clap(
			help = "Filter which files should be installed, checks if if path starts with this"
		)]
		filter: Option<String>,

		#[clap(short, long, default_value = "true", help = "Symlink the built files")]
		link: bool,
	},
	#[clap(about = "Display diff between repo and installed files")]
	Diff {
		#[clap(
			short,
			long,
			default_value = ".df",
			help = "Path to source repo, absolute path or path relative to $HOME"
		)]
		repo_path: PathBuf,

		#[clap(help = "Filter which files should be diffed, checks if if path starts with this")]
		filter: Option<String>,
	},
	#[clap(about = "Remove dotfiles")]
	Cleanup,
	#[clap(about = "Generate shell completions")]
	GenerateCompletions {
		#[clap(long, value_name = "SHELL", help = "Generate shell completions")]
		shell: Shell,
	},
}
