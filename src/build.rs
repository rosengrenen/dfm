use crate::{
	config::Config,
	utils::{get_tree_files, remove_dir_if_empty},
};

pub async fn build(config: &Config) -> anyhow::Result<()> {
	tokio::fs::create_dir_all(&config.build_dir).await?;
	let source_files = get_tree_files(config, &config.source_dir).await?;
	let build_files = get_tree_files(config, &config.build_dir).await?;

	for file_path in source_files.iter() {
		if let Some(folder_path) = file_path.parent() {
			let dir = config.build_dir.join(folder_path);
			tokio::fs::create_dir_all(dir).await?;
		}

		let from = config.source_dir.join(file_path);
		let to = config.build_dir.join(file_path);
		tokio::fs::copy(from, to).await?;
	}

	for file_path in build_files {
		if !source_files.contains(&file_path) {
			let file = config.build_dir.join(file_path);
			tokio::fs::remove_file(file.clone()).await?;
			remove_dir_if_empty(file.parent().unwrap()).await?;
		}
	}

	Ok(())
}
