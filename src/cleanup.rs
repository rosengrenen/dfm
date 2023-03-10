use std::io::BufRead;

use crate::{context::Context, utils::get_tree_files};

pub fn cleanup(context: &Context) -> anyhow::Result<()> {
	let installed_files = get_tree_files(context, &context.install_dir)?;
	log::debug!("Installed files: {:#?}", installed_files);

	let output = std::process::Command::new("find")
		.arg(&context.link_dir)
		.arg("-type")
		.arg("l")
		.output()?;
	let files = output
		.stdout
		.lines()
		.filter_map(|line| line.ok())
		.filter_map(|line| {
			let mut abs_path = context.link_dir.to_path_buf();
			abs_path.push(line);
			match abs_path.exists() {
				true => Some(abs_path),
				false => None,
			}
		})
		.collect::<Vec<_>>();
	for file in files {
		let actual_path = std::fs::canonicalize(&file)?;
		if actual_path.starts_with(&context.install_dir) {
			std::fs::remove_file(file)?;
		}
	}

	Ok(())
}
