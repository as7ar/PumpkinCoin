use pumpkin_plugin_api::Plugin;

struct PumpkinCoin;
impl Plugin for PumpkinCoin {
    fn new() -> Self {
        PumpkinCoin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "PumpkinCoin".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["AS7AR".into()],
            description: "An Economy API for PUMPKINMC".into(),
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
