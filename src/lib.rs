use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::info;

pub mod api;
pub mod economy_manager;
pub mod err;
pub mod utils;

pub use crate::api::*;
pub use crate::economy_manager::*;

struct PumpkinCoin;

impl Plugin for PumpkinCoin {
    fn new() -> Self {
        Self
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "PumpkinCoin".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["AS7AR".into()],
            description: "An Economy API for PUMPKINMC".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("PumpkinCoin Loaded");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("PumpkinCoin Unloaded");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PumpkinCoin);
