use colored::Colorize;

use crate::{
	context::Context,
	utils::{get_tree_files, remove_dir_if_empty},
};

pub fn apply(context: &Context, filter: &Option<String>, link: bool) -> anyhow::Result<()> {
	std::fs::create_dir_all(&context.install_dir)?;
	let mut built_files = get_tree_files(context, &context.build_dir)?;
	let mut installed_files = get_tree_files(context, &context.install_dir)?;
	if let Some(filter) = filter {
		log::debug!("Using filter: {}", filter);
		built_files = built_files
			.into_iter()
			.filter(|path| path.starts_with(filter))
			.collect();
		installed_files = installed_files
			.into_iter()
			.filter(|path| path.starts_with(filter))
			.collect();
	}

	log::debug!("Built files: {:#?}", built_files);
	log::debug!("Installed files: {:#?}", installed_files);

	for file_path in built_files.iter() {
		if let Some(folder_path) = file_path.parent() {
			let dir = context.install_dir.join(folder_path);
			std::fs::create_dir_all(dir)?;
		}

		let from = context.build_dir.join(file_path);
		let to = context.install_dir.join(file_path);
		std::fs::copy(from, to)?;

		if link {
			// Make sure symlink doesn't exist before attempting to symlink
			let link_target = context.link_dir.join(&file_path);
			let link_target_exists = link_target.exists();
			if link_target_exists && !link_target.is_symlink() {
				let error = format!(
					"Could not link {:?}, link target already exists and is not a symlink",
					file_path
				);
				println!("{}", error.red());
			} else {
				if link_target_exists {
					match std::fs::remove_file(&link_target) {
						Ok(_) => {}
						Err(e) => anyhow::bail!(e),
					};
				}

				if let Some(folder_path) = file_path.parent() {
					let dir = context.link_dir.join(folder_path);
					std::fs::create_dir_all(dir)?;
				}

				std::os::unix::fs::symlink(context.install_dir.join(&file_path), link_target)?;
			}
		}
	}

	for file_path in installed_files {
		if !built_files.contains(&file_path) {
			let installed_file = context.install_dir.join(&file_path);
			let linked_file = context.link_dir.join(&file_path);
			if linked_file.exists() {
				std::fs::remove_file(linked_file.clone())?;
				remove_dir_if_empty(linked_file.parent().unwrap())?;
			}

			if installed_file.exists() {
				std::fs::remove_file(installed_file.clone())?;
				remove_dir_if_empty(installed_file.parent().unwrap())?;
			}
		}
	}

	Ok(())
}
