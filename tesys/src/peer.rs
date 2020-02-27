use crate::{loggable, Loggable};
use crate::timing::LoopTimer;
use crate::PluginManager;
use crate::conf::{TesysConfiguration};

#[derive(Loggable)]
pub struct Peer {
    plugin_manager: PluginManager,
    config: TesysConfiguration,
    do_run: bool,
}

impl Peer {
    pub fn new() -> Peer {
        Peer {
            plugin_manager: PluginManager::new(),
            config: TesysConfiguration::new(),
            do_run:  false,
        }
    }

    pub fn load_plugins(&mut self) {
        self.plugin_manager
            .add_plugin_search_directory("./target/debug/");
        
        match &self.config.plugins {
            Some(plugins_confs) => {
                for (_k, pg) in plugins_confs {
                    let pg_res = self.plugin_manager.load(pg.instanceof.as_ref());
        
                    match pg_res {
                        Ok(_e) => (),
                        Err(_e) => tesys_err!(Self, "Unable to load plugin: {}", _e),
                    };
                }
            },
            None => println!("No plugins configured. ")
        }
    }

    pub fn set_config(&mut self, conf: TesysConfiguration) {
        self.config = conf;
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
