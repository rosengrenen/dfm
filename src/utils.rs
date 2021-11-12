use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

use futures::future::try_join_all;
use tokio::fs::read_dir;

use crate::config::Config;

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get_tree_files(
	config: &Config,
	tree_root: &Path,
) -> std::io::Result<HashSet<PathBuf>> {
	get_tree_files_recursively(config, tree_root, PathBuf::new()).await
}

#[async_recursion::async_recursion]
async fn get_tree_files_recursively(
	config: &Config,
	tree_root: &Path,
	relative_path: PathBuf,
) -> std::io::Result<HashSet<PathBuf>> {
	let current_path = tree_root.join(&relative_path);

	let mut dir_walker = read_dir(&current_path).await?;
	let mut dir_tasks = vec![];

	let mut files = HashSet::new();

	while let Some(entry) = dir_walker.next_entry().await? {
		let metadata = entry.metadata().await?;
		let os_name = entry.file_name();
		let name = os_name.to_string_lossy().to_string();
		if config.ignored_dirs.contains(&name) {
			continue;
		}

		if metadata.is_dir() {
			dir_tasks.push(get_tree_files_recursively(
				config,
				tree_root,
				relative_path.join(name),
			));
		} else if metadata.is_file() {
			files.insert(relative_path.join(name));
		}
	}

	let file_sets = try_join_all(dir_tasks).await?;
	for file_set in file_sets {
		files.extend(file_set);
	}

	Ok(files)
}

pub async fn remove_dir_if_empty(path: &Path) -> std::io::Result<()> {
	let mut dir_walker = tokio::fs::read_dir(path).await?;

	if dir_walker.next_entry().await?.is_none() {
		tokio::fs::remove_dir(path).await?;
	}

	Ok(())
}
