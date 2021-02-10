use crate::consts::DB_FILE_NAME;
use crate::errors::Result;
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn database_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or("unable to obtain database directory")?;

    let base_dir = dirs.data_dir().to_path_buf();
    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir)?;
    }

    Ok(base_dir.join(DB_FILE_NAME))
}
