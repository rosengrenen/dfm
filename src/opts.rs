use std::path::PathBuf;

use clap::Parser;
use clap_complete::Shell;

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
		help = "Absolute path or relative path to $HOME"
	)]
	pub repo_path: PathBuf,

	#[clap(long, value_name = "SHELL", help = "Generate shell completions")]
	pub generate: Option<Shell>,
}
