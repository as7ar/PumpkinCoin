use std::sync::{Arc, OnceLock, RwLock};

use crate::api::Economy;

type EconomyProvider = Arc<RwLock<Box<dyn Economy>>>;

static PROVIDER: OnceLock<RwLock<Option<EconomyProvider>>> = OnceLock::new();

pub struct EconomyManager;
impl EconomyManager {
    pub fn register(economy: Box<dyn Economy>) {
        let provider = PROVIDER.get_or_init(|| RwLock::new(None));

        *provider.write().unwrap() = Some(Arc::new(RwLock::new(economy)));
    }

    pub fn provider() -> Option<EconomyProvider> {
        PROVIDER
            .get()
            .and_then(|p| -> bool { p.read().unwrap().is_some().clone() })
    }

    pub fn is_registered() -> bool {
        PROVIDER.get().is_some_and(|p| p.read().unwrap().is_some())
    }
}
