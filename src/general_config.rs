use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing::metadata::LevelFilter;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::{filter, Registry};

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

pub static RELOAD_HANDLE: OnceLock<Handle<LevelFilter, Registry>> = OnceLock::new();
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    pub max_level: Option<TracingLevel>,
}

impl LoggingConfig {
    pub fn set_max_level(&mut self, tracing_level: TracingLevel) {
        self.max_level = Some(tracing_level);
        RELOAD_HANDLE
            .get()
            .unwrap()
            .reload(self.max_level.as_ref().unwrap().level())
            .unwrap();
    }
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
    pub fn from_str(string: &str) -> Result<Self> {
        Ok(match string {
            "Trace" => TracingLevel::Trace,
            "Debug" => TracingLevel::Debug,
            "Info" => TracingLevel::Info,
            "Warn" => TracingLevel::Warn,
            "Error" => TracingLevel::Error,
            _ => bail!("Invalid enum passed"),
        })
    }

    pub fn level(&self) -> filter::LevelFilter {
        match self {
            TracingLevel::Trace => LevelFilter::TRACE,
            TracingLevel::Debug => LevelFilter::DEBUG,
            TracingLevel::Info => LevelFilter::INFO,
            TracingLevel::Warn => LevelFilter::WARN,
            TracingLevel::Error => LevelFilter::ERROR,
        }
    }
}
