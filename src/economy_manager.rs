use std::sync::{Arc, OnceLock, RwLock};

use pumpkin_plugin_api::Plugin;

use crate::api::Economy;

pub struct ProcviderInfo<T: Plugin> {
    plugin: T,
    provider: EconomyProvider,
}

type EconomyProvider = Arc<RwLock<Box<dyn Economy + 'static>>>;

static PROVIDER: OnceLock<RwLock<Option<ProcviderInfo<T>>>> = OnceLock::new();

pub struct EconomyManager<T: Plugin>;
impl<T: Plugin> EconomyManager<T> {
    pub fn register(plugin: T, economy: Box<dyn Economy + 'static>) {
        let provider = PROVIDER.get_or_init(|| RwLock::new(None));

        *provider.write().unwrap() = Some(ProcviderInfo {
            plugin: plugin,
            provider: Arc::new(RwLock::new(economy)),
        });
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
