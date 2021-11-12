use std::{io::Write, path::PathBuf};

use terminal_size::{terminal_size, Width};

use crate::{config::Config, utils::get_tree_files};

pub async fn diff(config: &Config, diff_command: String) -> anyhow::Result<()> {
	let built_files = get_tree_files(config, &config.build_dir).await?;
	let installed_files = get_tree_files(config, &config.install_dir).await?;

	let mut all_files = built_files.clone();
	all_files.extend(installed_files.clone());
	let mut all_files: Vec<_> = all_files.iter().collect();
	all_files.sort();

	let (Width(terminal_width), _) = terminal_size().expect("Could not get terminal size");
	for file in all_files {
		let is_added = built_files.get(file).is_some() && installed_files.get(file).is_none();
		let is_removed = built_files.get(file).is_none() && installed_files.get(file).is_some();

		let (workdir, first_path, second_path) = if is_added {
			(
				config.build_dir.clone(),
				file.clone(),
				PathBuf::from("/dev/null"),
			)
		} else if is_removed {
			(
				config.install_dir.clone(),
				PathBuf::from("/dev/null"),
				file.clone(),
			)
		} else {
			(
				config.build_dir.clone(),
				file.clone(),
				config.install_dir.join(file),
			)
		};

		let output = tokio::process::Command::new(&diff_command)
			.current_dir(workdir)
			.arg(second_path)
			.arg(first_path)
			.arg("-w")
			.arg(terminal_width.to_string())
			.output()
			.await?;
		std::io::stdout().write_all(&output.stdout)?;
	}

	Ok(())
}
