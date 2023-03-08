use std::{io::Write, path::PathBuf};

use terminal_size::{terminal_size, Width};

use crate::{context::Context, utils::get_tree_files};

pub fn diff(context: &Context) -> anyhow::Result<()> {
	std::fs::create_dir_all(&context.install_dir)?;
	let built_files = get_tree_files(context, &context.build_dir)?;
	let installed_files = get_tree_files(context, &context.install_dir)?;
	log::debug!("Built files: {:#?}", built_files);
	log::debug!("Installed files: {:#?}", installed_files);

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
				context.build_dir.clone(),
				file.clone(),
				PathBuf::from("/dev/null"),
			)
		} else if is_removed {
			(
				context.install_dir.clone(),
				PathBuf::from("/dev/null"),
				file.clone(),
			)
		} else {
			(
				context.build_dir.clone(),
				file.clone(),
				context.install_dir.join(file),
			)
		};

		let output = std::process::Command::new("difft")
			.current_dir(workdir)
			.arg(second_path)
			.arg(first_path)
			.arg("--skip-unchanged")
			.arg("--color")
			.arg("always")
			.arg("--width")
			.arg(terminal_width.to_string())
			.output()?;
		std::io::stdout().write_all(&output.stdout)?;
	}

	Ok(())
}
