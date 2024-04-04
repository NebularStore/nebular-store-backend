use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminConfig {
    credentials: Credentials
}

#[allow(unused)]
impl AdminConfig {
    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }
}

impl AdminConfig {
    pub fn read() -> Result<AdminConfig> {
        let config_string = fs::read_to_string("./data/config/admin.toml")?;
        Ok(toml::from_str(&config_string)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    password: String
}