use crate::paths;
use crate::{consts::DEFAULT_DOWNLOAD_BASE_DIR, errors::Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub base_directory: PathBuf,
}

impl Config {
    pub fn load() -> Result<Config> {
        let config_path = paths::config_path()?;

        if !config_path.exists() {
            let default_config = Self::default_config()?;
            let config_buf = toml::to_vec(&default_config)?;
            fs::write(&config_path, config_buf.as_slice())?;

            return Ok(default_config);
        }

        let config_buf = fs::read(config_path)?;

        Ok(toml::from_slice(config_buf.as_slice())?)
    }

    fn default_config() -> Result<Config> {
        let base_path = paths::base_dir()?;

        Ok(Config {
            base_directory: base_path.join(DEFAULT_DOWNLOAD_BASE_DIR),
        })
    }
}
