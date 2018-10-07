extern crate libloading;
use self::libloading::{Library, Symbol};

use loggable::*;
use Plugin;

const _plugin_create_symbol: &[u8] = b"_create_plugin";
const _plugin_destroy_symbol: &[u8] = b"_destroy_symbol";

#[derive(Loggable)]
pub struct PluginManager {
	loaded_plugins: Vec<Box<Plugin>>,
	loaded_libraries: Vec<Library>,
}

impl PluginManager {
	pub fn new() -> PluginManager {
		let pl = PluginManager {
			loaded_plugins: Vec::new(),
			loaded_libraries: Vec::new(),
		};

		
		pl // Return our new plugin manager instance
	}

	pub unsafe fn load(&mut self, path:String) -> Result<Box<Plugin>, &'static str>{
		type PluginCreate = unsafe fn() -> *mut Plugin;

		Self::log(&format!("Loading Plugin: {}", path));

		let lib = Library::new(path);
		match lib {
			Ok(l) => { 
				self.loaded_libraries.push(l);
				let lib = self.loaded_libraries.last().unwrap();
				match lib.get(_plugin_create_symbol) {
					Ok(sym) => {
						let func = sym as Symbol<PluginCreate>;
						let pg_raw = func();
						let pg = Box::from_raw(pg_raw);
						Ok(pg)
					},
					Err(e) => Err("Unable to load _create symbol"),
				}
			},
			Err(e) => Err("Unable to load library file"),
		}
	}
}