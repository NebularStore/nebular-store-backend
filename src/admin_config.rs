use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminConfig {
    pub credentials: Credentials,
}

impl AdminConfig {
    pub fn read() -> Result<AdminConfig> {
        let config_string = fs::read_to_string("./data/config/admin.toml")?;
        let mut config: AdminConfig = toml::from_str(&config_string)?;
        config.credentials.hash = sha256::digest(config.credentials.password.clone());
        Ok(config)
    }

    pub fn check_admin_hash(&self, password_hash: String) -> bool {
        password_hash.eq(&self.credentials.hash)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    password: String,
    #[serde(skip_deserializing)]
    hash: String,
}
