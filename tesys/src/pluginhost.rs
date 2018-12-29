use std::{thread, time};

extern crate libloading;
use self::libloading::{Library, Symbol};

use crate::loggable;
use crate::Plugin;
use crate::plugin;
use crate::Exchange;

pub enum RunMode {
	Thread,
	Process,
}

const DEFAULT_RUN_MODE: RunMode = RunMode::Thread;

pub struct PluginHost {
	pub pg: Option<Box<Plugin>>,
	ex: Exchange,
	run_mode: RunMode,
	do_run: bool,
	library: Option<Library>,
}

impl PluginHost {

	pub fn new() -> PluginHost {
		PluginHost {
			pg: None,
			ex: Exchange::new(),
			run_mode: DEFAULT_RUN_MODE,
			do_run: false,
			library: None
		}
	}

	pub fn set_run_mode(&mut self, rm: RunMode) {
		self.run_mode = rm;
	}

	pub fn set_plugin(&mut self, pg: Box<Plugin>) {
		self.pg = Some(pg);
	}

	pub fn init(&self) {

	}

	pub fn start(&mut self) {
		self.do_run = true;
	}

	pub fn main(&self) {

	}

	pub fn test(&mut self) {
		match &mut self.pg {
			None => (),
			Some(p) => p.test(),
		};
	}

	pub fn load(path: String) -> Result<PluginHost, String> {
		let mut pgh = PluginHost::new();
		unsafe{ 
            match pgh._load_plugin(path.clone()) {
                Ok(_pg) => {
                    Ok(pgh)
                },
                Err(_e) => Err(format!("Unable to load plugin '{}'", path).to_string())
            } 
        }
    }

    unsafe fn _load_plugin(&mut self, path: String) -> Result<(), String> {

        tesys_log!("Loading Plugin: {}", path);

        let lib = Library::new(path);
        match lib {
            Ok(l) => {
                self.library = Some(l);
                match &self.library {
                	None => Err(format!("Unable to get _create_plugin() from plugin: Library property of PluginHost is None. ").to_string()),
                	Some(lib) => match lib.get(plugin::PLUGIN_CREATE_SYMBOL) {
	                    Ok(sym) => {
	                        let func = sym as Symbol<plugin::PluginCreate>;
	                        let pg_raw = func();
	                        self.pg = Some(Box::from_raw(pg_raw));
	                        Ok(())
	                    }
	                    Err(_e) => Err(format!("Unable to load _create symbol: {}", _e).to_string()),
	                }
                }
            }
            Err(_e) => Err(format!("Unable to load library file: {}", _e).to_string()),
        }
    }
}