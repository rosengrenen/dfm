use crate::{
	config::Config,
	utils::{get_tree_files, remove_dir_if_empty},
};

pub async fn install(config: &Config) -> std::io::Result<()> {
	let built_files = get_tree_files(config, &config.build_dir).await?;
	let installed_files = get_tree_files(config, &config.install_dir).await?;

	for file_path in built_files.iter() {
		if let Some(folder_path) = file_path.parent() {
			let dir = config.build_dir.join(folder_path);
			tokio::fs::create_dir_all(dir).await?;
		}

		let from = config.source_dir.join(file_path);
		let to = config.build_dir.join(file_path);
		tokio::fs::copy(from, to).await?;

		if let Some(folder_path) = file_path.parent() {
			let dir = config.link_dir.join(folder_path);
			tokio::fs::create_dir_all(dir).await?;
		}

		tokio::fs::symlink(
			config.install_dir.join(&file_path),
			config.link_dir.join(&file_path),
		)
		.await?;
	}

	for file_path in installed_files {
		if !built_files.contains(&file_path) {
			let installed_file = config.install_dir.join(&file_path);
			let linked_file = config.link_dir.join(&file_path);
			tokio::fs::remove_file(linked_file.clone()).await?;
			remove_dir_if_empty(linked_file.parent().unwrap()).await?;
			tokio::fs::remove_file(installed_file.clone()).await?;
			remove_dir_if_empty(installed_file.parent().unwrap()).await?;
		}
	}

	Ok(())
}
