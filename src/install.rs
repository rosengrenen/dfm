use crate::{
	config::Config,
	utils::{get_tree_files, remove_dir_if_empty},
};

pub async fn install(config: &Config) -> anyhow::Result<()> {
	tokio::fs::create_dir_all(&config.install_dir).await?;
	let built_files = get_tree_files(config, &config.build_dir).await?;
	let installed_files = get_tree_files(config, &config.install_dir).await?;

	for file_path in built_files.iter() {
		if let Some(folder_path) = file_path.parent() {
			let dir = config.install_dir.join(folder_path);
			tokio::fs::create_dir_all(dir).await?;
		}

		let from = config.build_dir.join(file_path);
		let to = config.install_dir.join(file_path);
		tokio::fs::copy(from, to).await?;

		if let Some(folder_path) = file_path.parent() {
			let dir = config.link_dir.join(folder_path);
			tokio::fs::create_dir_all(dir).await?;
		}

		// Make sure symlink doesn't exist before attempting to symlink
		let link_target = config.link_dir.join(&file_path);
		match tokio::fs::remove_file(&link_target).await {
			Ok(_) => {}
			Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
			Err(e) => anyhow::bail!(e),
		};
		tokio::fs::symlink(config.install_dir.join(&file_path), link_target).await?;
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
