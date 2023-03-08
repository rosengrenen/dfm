use crate::{
	context::Context,
	utils::{get_tree_files, remove_dir_if_empty},
};

pub fn build(context: &Context) -> anyhow::Result<()> {
	std::fs::create_dir_all(&context.build_dir)?;
	let source_files = get_tree_files(context, &context.source_dir)?;
	let build_files = get_tree_files(context, &context.build_dir)?;
	log::debug!("Source files: {:#?}", source_files);
	log::debug!("Existing files: {:#?}", build_files);

	for file_path in source_files.iter() {
		// Make sure that the folder into which the file is to be created exists
		if let Some(folder_path) = file_path.parent() {
			let dir = context.build_dir.join(folder_path);
			std::fs::create_dir_all(dir)?;
		}

		let from = context.source_dir.join(file_path);
		let to = context.build_dir.join(file_path);
		log::debug!("Copied {:?} to {:?}", from, to);
		std::fs::copy(from, to)?;
	}

	for file_path in build_files {
		if !source_files.contains(&file_path) {
			let file = context.build_dir.join(file_path);
			log::debug!("Removed {:?}", file);
			std::fs::remove_file(file.clone())?;
			remove_dir_if_empty(file.parent().unwrap())?;
		}
	}

	Ok(())
}
