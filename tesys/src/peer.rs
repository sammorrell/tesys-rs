use loggable::*;
use PluginManager;

#[derive(Loggable)]
pub struct Peer {
	plugin_manager: PluginManager
}

impl Peer {
    pub fn new() -> Peer {
        Peer {
        	plugin_manager: PluginManager::new()
        }
    }

    pub fn load_plugins(&mut self) {
        unsafe {
            let pg = self.plugin_manager.load("target/debug/libtesys_example_plugin.dylib".to_string());

            match pg {
                Ok(mut p) => p.test(),
                Err(e) => Self::err(&format!("Unable to load plugin: {}. ", e)),
            }
        }
    }
}
