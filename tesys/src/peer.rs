use loggable;
use loggable::Loggable;
use PluginManager;

#[derive(Loggable)]
pub struct Peer {
    plugin_manager: PluginManager,
}

impl Peer {
    pub fn new() -> Peer {
        Peer {
            plugin_manager: PluginManager::new(),
        }
    }

    pub fn load_plugins(&mut self) {
        self.plugin_manager.add_plugin_search_directory("./target/debug/");
        let pg = self.plugin_manager.load("tesys_example_plugin");

        match pg {
            Ok(p) => p.test(),
            Err(e) => tesys_err!("Unable to load plugin: {}. ", e),
        }
    }
}
