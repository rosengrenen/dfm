use std::path::PathBuf;

use async_recursion::async_recursion;
use futures::future::join_all;
use tokio::fs::{copy, create_dir, read_dir};

use crate::Context;

pub async fn build_tree(context: &Context) -> std::io::Result<()> {
    dir(context, PathBuf::new()).await
}

#[async_recursion]
async fn dir(context: &Context, relative_path: PathBuf) -> std::io::Result<()> {
    let source_path = context.tree_source_dir.join(&relative_path);
    let build_path = context.tree_build_dir.join(&relative_path);

    match create_dir(build_path).await {
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => {
            return Err(e);
        }
        _ => (),
    }

    let mut dir_walker = read_dir(source_path).await?;

    let mut dir_tasks = vec![];
    let mut file_tasks = vec![];

    while let Some(entry) = dir_walker.next_entry().await? {
        let metadata = entry.metadata().await?;
        let os_name = entry.file_name();
        let name = os_name.to_str();
        if name == Some(".git") {
            continue;
        }

        let relative_path = relative_path.join(entry.file_name());

        if metadata.is_dir() {
            dir_tasks.push(dir(context, relative_path));
        } else if metadata.is_file() {
            file_tasks.push(file(context, relative_path));
        }
    }

    let dir_tasks = async {
        join_all(dir_tasks)
            .await
            .into_iter()
            .collect::<std::io::Result<Vec<_>>>()
    };
    let file_tasks = async {
        join_all(file_tasks)
            .await
            .into_iter()
            .collect::<std::io::Result<Vec<_>>>()
    };

    tokio::try_join!(dir_tasks, file_tasks)?;

    Ok(())
}

async fn file(context: &Context, relative_path: PathBuf) -> std::io::Result<()> {
    let source_path = context.tree_source_dir.join(&relative_path);
    let build_path = context.tree_build_dir.join(&relative_path);

    copy(source_path, build_path).await?;
    Ok(())
}
