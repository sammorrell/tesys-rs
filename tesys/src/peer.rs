use crate::loggable;
use crate::loggable::Loggable;
use crate::PluginManager;

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
        let pg_res = self.plugin_manager.load("tesys_example_plugin");

        match pg_res {
            Ok(_e) => (),
            Err(_e) => println!("Unable to load plugin: {}", _e),
        };
    }
}
