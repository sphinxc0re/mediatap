use crate::consts::DB_FILE_NAME;
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn database_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")).unwrap();

    let base_dir = dirs.data_dir().to_path_buf();
    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir)?;
    }

    Ok(base_dir.join(DB_FILE_NAME))
}