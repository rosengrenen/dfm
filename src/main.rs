mod build;
mod check;
mod link;
mod tree;

use std::path::PathBuf;

use build::build_tree;
use check::check_tree;
use link::link_tree;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = clap::App::new("DotFiles Manager")
        .version(APP_VERSION)
        .author("Rasmus Rosengren <rasmus.rosengren@protonmail.com>")
        .about("Utility to manage dotfiles")
        .arg(
            clap::Arg::with_name("dry-run")
                .long("dry-run")
                .short("d")
                .help("Report changes without installing"),
        )
        .arg(
            clap::Arg::with_name("repo-path")
                .long("repo-path")
                .short("r")
                .default_value(".df")
                .help("Location of repo, relative to $HOME"),
        );

    let matches = app.get_matches();

    let home_dir = std::env::var("HOME").expect("$HOME variable not found");
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dfm").expect("xdg dirs");

    let context = Context {
        tree_source_dir: format!("{}/{}", home_dir, matches.value_of("repo-path").unwrap()).into(),
        tree_build_dir: xdg_dirs
            .create_cache_directory("tree")
            .expect("xdg cache dir"),
        tree_install_dir: xdg_dirs
            .create_config_directory("tree")
            .expect("xdg config dir"),
        link_root_dir: home_dir.into(),
    };

    build_tree(&context).await?;
    if matches.is_present("dry-run") {
        let (add, change, delete) = check_tree(&context).await?;
        if !add.is_empty() {
            println!("The following files have been added:");
            for file in add {
                println!("\t{}", file.to_string_lossy());
            }
            println!();
        }

        if !change.is_empty() {
            println!("The following files have been changed:");
            for file in change {
                println!("\t{}", file.to_string_lossy());
            }
            println!();
        }

        if !delete.is_empty() {
            println!("The following files have been deleted:");
            for file in delete {
                println!("\t{}", file.to_string_lossy());
            }
            println!();
        }
    } else {
        link_tree(&context).await?;
    }

    Ok(())
}

pub struct Context {
    tree_source_dir: PathBuf,
    tree_build_dir: PathBuf,
    tree_install_dir: PathBuf,
    link_root_dir: PathBuf,
}
