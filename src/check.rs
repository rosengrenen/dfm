use std::path::PathBuf;

use tokio::fs::read;

use crate::{tree::get_tree_files, Context};

pub async fn check_tree(
    context: &Context,
) -> std::io::Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>)> {
    let source_files = get_tree_files(&context.tree_source_dir).await?;
    let installed_files = get_tree_files(&context.tree_install_dir).await?;

    let mut deleted_files = vec![];
    let mut changed_files = vec![];
    for installed_file in installed_files.iter() {
        if source_files.contains(installed_file) {
            let source_file_content = read(context.tree_build_dir.join(installed_file)).await?;
            let installed_file_content =
                read(context.tree_install_dir.join(installed_file)).await?;
            if source_file_content != installed_file_content {
                changed_files.push(installed_file.clone());
            }
        } else {
            deleted_files.push(installed_file.clone());
        }
    }

    let mut added_files = vec![];
    for source_file in source_files.iter() {
        if !installed_files.contains(source_file) {
            added_files.push(source_file.clone());
        }
    }

    Ok((added_files, changed_files, deleted_files))
}
