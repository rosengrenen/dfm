use std::path::PathBuf;

use clap::Parser;
use clap_generate::Shell;

use crate::utils::APP_VERSION;

#[derive(Debug, Parser)]
#[clap(
	name = "dfm",
	version = APP_VERSION,
	author = "Rasmus Rosengren <rasmus.rosengren@protonmail.com>",
	about = "Utility to manage dotfiles"
)]
pub struct Opts {
	#[clap(short, long)]
	pub install: bool,

	#[clap(
		short,
		long,
		default_value = ".df",
		about = "Absolute path or relative path to $HOME"
	)]
	pub repo_path: PathBuf,

	#[clap(
		short,
		long,
		default_value = "delta",
		about = "Command to execute to show diffs"
	)]
	pub diff_command: String,

	#[clap(long, value_name = "SHELL", about = "Generate shell completions")]
	pub generate: Option<Shell>,
}
