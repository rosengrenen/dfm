mod apply;
mod build;
mod context;
mod diff;
mod opts;
mod utils;

use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use directories::{ProjectDirs, UserDirs};

use crate::{
	apply::apply, build::build, context::Context, diff::diff, opts::Opts, utils::command_exists,
};

fn main() -> anyhow::Result<()> {
	pretty_env_logger::init();

	let opts = Opts::parse();
	log::debug!("Command line options: {:#?}", opts);
	match opts.command {
		opts::Commands::Apply {
			repo_path,
			filter,
			link,
		} => {
			if !command_exists("difft") {
				println!("'difft' is not installed");
			} else {
				let context = build_context(repo_path);
				log::debug!("Context: {:#?}", context);
				build(&context).expect("Failed to build");
				apply(&context, &filter, link).expect("Failed to apply");
			}
		}
		opts::Commands::Diff { repo_path } => {
			let context = build_context(repo_path);
			log::debug!("Context: {:#?}", context);
			build(&context).expect("Failed to build");
			diff(&context).expect("Failed to diff");
		}
		opts::Commands::GenerateCompletions { shell } => {
			generate(shell, &mut Opts::command(), "dfm", &mut std::io::stdout());
		}
	}

	Ok(())
}

fn build_context(repo_path: PathBuf) -> Context {
	let user_dirs = UserDirs::new().expect("Could not find user directories");
	let project_dirs =
		ProjectDirs::from("se", "rsrp", "dfm").expect("Could not find project directories");

	let repo_path = if repo_path.is_relative() {
		let mut absolute_repo_path = PathBuf::from(user_dirs.home_dir());
		absolute_repo_path.push(repo_path);
		absolute_repo_path
	} else {
		repo_path
	};

	Context {
		source_dir: repo_path,
		build_dir: project_dirs.cache_dir().to_path_buf().join("tree"),
		install_dir: project_dirs.config_dir().to_path_buf().join("tree"),
		link_dir: user_dirs.home_dir().to_path_buf(),
		ignored_dirs: vec![".git".to_string()],
	}
}
