use crate::general_config::GeneralConfig;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::admin_config::AdminConfig;

pub type SharedState = Arc<RwLock<State>>;

pub struct State {
    pub general_config: GeneralConfig,
    pub admin_config: AdminConfig,
}

pub fn init_state() -> Result<SharedState> {
    Ok(Arc::new(RwLock::new(State {
        general_config: GeneralConfig::read()?,
        admin_config: AdminConfig::read()?,
    })))
}
