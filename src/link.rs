use std::path::PathBuf;

use tokio::fs::{copy, create_dir_all, remove_file, symlink};

use crate::{check::check_tree, Context};

pub async fn link_tree(context: &Context) -> std::io::Result<()> {
    let (add, update, remove) = check_tree(context).await?;
    for file in add {
        copy_and_link(context, file).await?;
    }

    for file in update {
        copy_and_link(context, file).await?;
    }

    for file in remove {
        remove_file(context.link_root_dir.join(file)).await?;
    }

    Ok(())
}

async fn copy_and_link(context: &Context, relative_path: PathBuf) -> std::io::Result<()> {
    // Create all necessary directories that are accessed
    let mut current_install_dir = context.tree_install_dir.join(&relative_path);
    current_install_dir.pop();
    let mut current_link_dir = context.link_root_dir.join(&relative_path);
    current_link_dir.pop();
    create_dir_all(current_install_dir).await?;
    create_dir_all(current_link_dir).await?;

    // Copy from build to install
    copy(
        context.tree_build_dir.join(&relative_path),
        context.tree_install_dir.join(&relative_path),
    )
    .await?;

    // Make sure symlink doesn't exist before attempting to symlink
    match remove_file(context.link_root_dir.join(&relative_path)).await {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    };
    symlink(
        context.tree_install_dir.join(&relative_path),
        context.link_root_dir.join(&relative_path),
    )
    .await?;
    Ok(())
}
