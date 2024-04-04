use crate::general_config::GeneralConfig;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::admin_config::AdminConfig;

pub type SharedState = Arc<RwLock<State>>;

pub struct State {
    general_config: GeneralConfig,
    admin_config: AdminConfig,
}

impl State {
    pub fn general_config(&self) -> &GeneralConfig {
        &self.general_config
    }


    pub fn admin_config(&self) -> &AdminConfig {
        &self.admin_config
    }
}

pub fn init_state() -> Result<SharedState> {
    Ok(Arc::new(RwLock::new(State {
        general_config: GeneralConfig::read()?,
        admin_config: AdminConfig::read()?,
    })))
}
