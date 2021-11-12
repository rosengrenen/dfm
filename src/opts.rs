use std::path::PathBuf;

use clap::Parser;

use crate::utils::APP_VERSION;

#[derive(Debug, Parser)]
#[clap(
	version = APP_VERSION,
	author = "Rasmus Rosengren <rasmus.rosengren@protonmail.com>",
	about = "Utility to manage dotfiles"
)]
pub struct Opts {
	#[clap(short, long)]
	pub install: bool,

	#[clap(short, long, default_value = ".df")]
	pub repo_path: PathBuf,

	#[clap(short, long, default_value = "delta")]
	pub diff_command: String,
}
