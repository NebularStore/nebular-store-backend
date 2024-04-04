use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralConfig {
    pub server: ServerConfig,
    #[allow(unused)]
    pub theme: ThemeConfig,
    pub logging: LoggingConfig,
}

impl GeneralConfig {
    pub fn read() -> Result<GeneralConfig> {
        let config_string = fs::read_to_string("./data/config/general.toml")?;
        Ok(toml::from_str(&config_string)?)
    }

    pub fn write(&self) -> Result<()> {
        Ok(fs::write(
            "./data/config/general.toml",
            toml::to_string(self)?.as_bytes(),
        )?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeConfig {
    pub company_name: String,
    pub icon_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    pub max_level: Option<TracingLevel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl TracingLevel {
    pub fn level(self) -> tracing::Level {
        match self {
            TracingLevel::Trace => tracing::Level::TRACE,
            TracingLevel::Debug => tracing::Level::DEBUG,
            TracingLevel::Info => tracing::Level::INFO,
            TracingLevel::Warn => tracing::Level::WARN,
            TracingLevel::Error => tracing::Level::ERROR,
        }
    }
}
