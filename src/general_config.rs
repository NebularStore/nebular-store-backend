use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct GeneralConfig {
    server: ServerConfig,
    theme: ThemeConfig,
}

impl GeneralConfig {
    pub fn from_config_file<P: AsRef<Path>>(path: P) -> Result<GeneralConfig> {
        let config_string = fs::read_to_string(path)?;
        Ok(toml::from_str(&config_string)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    port: u32,
}

#[derive(Deserialize, Debug)]
pub struct ThemeConfig {
    company_name: String,
    icon_path: PathBuf,
}
