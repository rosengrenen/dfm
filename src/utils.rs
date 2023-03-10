use std::{
	collections::HashSet,
	path::{Path, PathBuf},
};

use crate::context::Context;

pub fn get_tree_files(context: &Context, tree_root: &Path) -> anyhow::Result<HashSet<PathBuf>> {
	get_tree_files_recursively(context, tree_root, PathBuf::new())
}

fn get_tree_files_recursively(
	context: &Context,
	tree_root: &Path,
	relative_path: PathBuf,
) -> anyhow::Result<HashSet<PathBuf>> {
	let current_path = tree_root.join(&relative_path);
	let mut dir_walker = match std::fs::read_dir(&current_path) {
		Ok(dir_walker) => dir_walker,
		Err(error) => match error.kind() {
			std::io::ErrorKind::PermissionDenied => return Ok(HashSet::new()),
			_ => anyhow::bail!(error),
		},
	};

	let mut files = HashSet::new();
	while let Some(Ok(entry)) = dir_walker.next() {
		let metadata = entry.metadata()?;
		let os_name = entry.file_name();
		let name = os_name.to_string_lossy().to_string();
		if context.ignored_dirs.contains(&name) {
			continue;
		}

		if metadata.is_dir() {
			files.extend(get_tree_files_recursively(
				context,
				tree_root,
				relative_path.join(name),
			)?);
		} else if metadata.is_file() {
			files.insert(relative_path.join(name));
		}
	}

	Ok(files)
}

pub fn remove_dir_if_empty(path: &Path) -> anyhow::Result<()> {
	let mut path = path.to_path_buf();

	loop {
		let mut dir_walker = std::fs::read_dir(&path)?;
		if dir_walker.next().is_none() {
			std::fs::remove_dir(&path)?;
			match path.parent() {
				Some(parent) => path = parent.to_path_buf(),
				None => break,
			}
		} else {
			break;
		}
	}

	Ok(())
}

pub fn command_exists(command: &str) -> bool {
	std::process::Command::new("which")
		.arg(&command)
		.stdout(std::process::Stdio::null())
		.stderr(std::process::Stdio::null())
		.status()
		.is_ok()
}
