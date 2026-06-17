use std::sync::{Arc, OnceLock, RwLock};

use pumpkin_plugin_api::Plugin;

use crate::api::Economy;

pub struct ProcviderInfo {
    plugin: Plugin,
    provider: EconomyProvider,
}

type EconomyProvider = Arc<RwLock<Box<dyn Economy>>>;

static PROVIDER: OnceLock<RwLock<Option<ProcviderInfo>>> = OnceLock::new();

pub struct EconomyManager;
impl EconomyManager {
    pub fn register(plugin: Plugin, economy: Box<dyn Economy>) {
        let provider = PROVIDER.get_or_init(|| RwLock::new(None));

        *provider.write().unwrap() = Some(ProcviderInfo(plugin, Arc::new(RwLock::new(economy))));
    }

    pub fn provider() -> Option<EconomyProvider> {
        PROVIDER
            .get()
            .and_then(|p| p.read().unwrap().as_ref().map(|p| p.provider.clone()))
    }

    pub fn is_registered() -> bool {
        PROVIDER.get().is_some_and(|p| p.read().unwrap().is_some())
    }
}
