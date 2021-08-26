use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use async_recursion::async_recursion;
use futures::future::try_join_all;
use tokio::fs::read_dir;

pub async fn get_tree_files(tree_root_path: &Path) -> std::io::Result<HashSet<PathBuf>> {
    dir(tree_root_path, PathBuf::new()).await
}

#[async_recursion]
async fn dir(tree_root_path: &Path, relative_path: PathBuf) -> std::io::Result<HashSet<PathBuf>> {
    let current_path = tree_root_path.join(&relative_path);

    let mut dir_walker = read_dir(&current_path).await?;
    let mut dir_tasks = vec![];

    let mut files = HashSet::new();

    while let Some(entry) = dir_walker.next_entry().await? {
        let metadata = entry.metadata().await?;
        let os_name = entry.file_name();
        let name = os_name.to_string_lossy().to_string();
        if name == ".git" {
            continue;
        }

        if metadata.is_dir() {
            dir_tasks.push(dir(tree_root_path, relative_path.join(name)));
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
