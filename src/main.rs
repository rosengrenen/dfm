mod build;
mod config;
mod diff;
mod install;
mod opts;
mod utils;

use std::path::PathBuf;

use clap::Parser;
use directories::{ProjectDirs, UserDirs};

use crate::{
	build::build, config::Config, diff::diff, install::install, opts::Opts,
	utils::remove_dir_if_empty,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let opts = Opts::parse();

	let user_dirs = UserDirs::new().expect("Could not find user directories");
	let project_dirs =
		ProjectDirs::from("se", "rsrp", "dfm").expect("Could not find project directories");

	let repo_path = if opts.repo_path.is_relative() {
		let mut repo_path = PathBuf::from(user_dirs.home_dir());
		repo_path.push(opts.repo_path);
		repo_path
	} else {
		opts.repo_path
	};

	let config = Config {
		source_dir: repo_path,
		build_dir: project_dirs.cache_dir().to_path_buf().join("tree"),
		install_dir: project_dirs.config_dir().to_path_buf().join("tree"),
		link_dir: user_dirs.home_dir().to_path_buf(),
		ignored_dirs: vec![".git".to_string()],
	};

	remove_dir_if_empty(&PathBuf::from("/home/rosen/test")).await?;
	build(&config).await?;
	if opts.install {
		install(&config).await?;
	} else {
		diff(&config, opts.diff_command).await?;
	}

	Ok(())
}
