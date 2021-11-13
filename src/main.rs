mod build;
mod config;
mod diff;
mod install;
mod opts;
mod utils;

use std::path::PathBuf;

use clap::{IntoApp, Parser};
use clap_generate::generate;
use directories::{ProjectDirs, UserDirs};

use crate::{build::build, config::Config, diff::diff, install::install, opts::Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let opts = Opts::parse();

	if let Some(generator) = opts.generate {
		generate(
			generator,
			&mut Opts::into_app(),
			"dfm",
			&mut std::io::stdout(),
		);
		return Ok(());
	}

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

	build(&config).await?;
	if opts.install {
		install(&config).await?;
	} else {
		diff(&config, opts.diff_command).await?;
	}

	Ok(())
}
