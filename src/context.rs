use std::path::PathBuf;

#[derive(Debug)]
pub struct Context {
	pub source_dir: PathBuf,
	pub build_dir: PathBuf,
	pub install_dir: PathBuf,
	pub link_dir: PathBuf,
	pub ignored_dirs: Vec<String>,
}
