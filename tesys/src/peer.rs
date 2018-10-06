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
}
