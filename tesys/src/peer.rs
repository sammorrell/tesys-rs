use crate::loggable::Loggable;
use crate::timing::LoopTimer;
use crate::PluginManager;

#[derive(Loggable)]
pub struct Peer {
    plugin_manager: PluginManager,

    do_run: bool,
}

impl Peer {
    pub fn new() -> Peer {
        Peer {
            plugin_manager: PluginManager::new(),
            do_run: false,
        }
    }

    pub fn load_plugins(&mut self) {
        self.plugin_manager
            .add_plugin_search_directory("./target/debug/");
        let pg_res = self.plugin_manager.load("tesys_example_plugin");

        match pg_res {
            Ok(_e) => (),
            Err(_e) => println!("Unable to load plugin: {}", _e),
        };
    }

    pub fn run(&mut self) -> Result<(), ()>{
        self.do_run = true;
        let mut lt = LoopTimer::new(60); // Target polling rate of 60 iter / sec.

        while self.do_run {
            lt.start();
            lt.end();
        }
        Ok(())
    }
}
