use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminConfig {
    pub credentials: Credentials,
}

impl AdminConfig {
    pub fn read() -> Result<AdminConfig> {
        let config_string = fs::read_to_string("./data/config/admin.toml")?;
        let mut config: AdminConfig = toml::from_str(&config_string)?;
        config.credentials.set_hash();
        Ok(config)
    }

    pub fn write(&self) -> Result<()> {
        Ok(fs::write(
            "./data/config/admin.toml",
            toml::to_string(self)?.as_bytes(),
        )?)
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

impl Credentials {
    pub fn set_password(&mut self, password: String) {
        self.password = password;
        self.set_hash();
    }
    
    pub fn set_hash(&mut self) {
        self.hash = sha256::digest(self.password.clone());
    }
}
