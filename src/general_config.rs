use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct GeneralConfig {
    server: ServerConfig,
    #[allow(unused)]
    theme: ThemeConfig,
}

impl GeneralConfig {
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
    #[allow(unused)]
    pub fn theme(&self) -> &ThemeConfig {
        &self.theme
    }
}

impl GeneralConfig {
    pub fn read() -> Result<GeneralConfig> {
        let config_string = fs::read_to_string("./data/config/general.toml")?;
        Ok(toml::from_str(&config_string)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    port: u16,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Deserialize, Debug)]
pub struct ThemeConfig {
    company_name: String,
    icon_path: PathBuf,
}

#[allow(unused)]
impl ThemeConfig {
    pub fn company_name(&self) -> &str {
        &self.company_name
    }
    pub fn icon_path(&self) -> &PathBuf {
        &self.icon_path
    }
}
