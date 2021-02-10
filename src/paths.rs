use crate::consts::{CONFIG_FILE_NAME, DB_FILE_NAME, SUBSCRIPTIONS_DIR_NAME};
use crate::errors::Result;
use directories::ProjectDirs;
use std::path::PathBuf;

fn project_dirs() -> Result<ProjectDirs> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .ok_or("unable to obtain database directory")?;

    Ok(dirs)
}

pub fn base_dir() -> Result<PathBuf> {
    let dirs = project_dirs()?;

    let base_dir = dirs.data_dir().to_path_buf();
    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir)?;
    }

    Ok(base_dir)
}

pub fn database_path() -> Result<PathBuf> {
    let base_dir = base_dir()?;

    Ok(base_dir.join(DB_FILE_NAME))
}

pub fn subscriptions_dir() -> Result<PathBuf> {
    let base_dir = base_dir()?;

    let path = base_dir.join(SUBSCRIPTIONS_DIR_NAME);
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn config_path() -> Result<PathBuf> {
    let base_dir = base_dir()?;

    let path = base_dir.join(CONFIG_FILE_NAME);
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}
