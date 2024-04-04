use crate::general_config::GeneralConfig;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedState = Arc<RwLock<State>>;

pub struct State {
    general_config: GeneralConfig,
}

impl State {
    pub fn general_config(&self) -> &GeneralConfig {
        &self.general_config
    }
}

pub fn init_state() -> Result<SharedState> {
    Ok(Arc::new(RwLock::new(State {
        general_config: GeneralConfig::read()?,
    })))
}
