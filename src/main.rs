use async_recursion::async_recursion;
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let home_dir = std::env::var("HOME").expect("Could not get home dir");
    symlink_folder(&format!("{}/.df", home_dir), &home_dir).await;
    Ok(())
}

#[async_recursion]
async fn symlink_folder(src_dir: &str, target_dir: &str) {
    fs::create_dir_all(target_dir)
        .await
        .expect(&format!("Could not create {}", target_dir));
    let mut dir_contents = fs::read_dir(src_dir)
        .await
        .expect(&format!("Could read dir {}", src_dir));
    while let Ok(Some(entry)) = dir_contents.next_entry().await {
        let os_name = entry.file_name();
        let name = os_name.to_str().unwrap();
        if name == ".git" {
            continue;
        }

        if let Ok(metadata) = entry.metadata().await {
            if metadata.is_dir() {
                symlink_folder(
                    &format!("{}/{}", src_dir, name),
                    &format!("{}/{}", target_dir, name),
                )
                .await;
            }

            if metadata.is_file() {
                #[allow(unused_must_use)]
                let _ = fs::remove_file(&format!("{}/{}", target_dir, name)).await;
                fs::symlink(
                    &format!("{}/{}", src_dir, name),
                    &format!("{}/{}", target_dir, name),
                )
                .await
                .unwrap();
            }
        }
    }
}
