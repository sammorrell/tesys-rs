extern crate libloading;
use self::libloading::{Library, Symbol};

use loggable::*;
use Plugin;

const _PLUGIN_CREATE_SYMBOL: &[u8] = b"_create_plugin";
const _PLUGIN_DESTROY_SYMBOL: &[u8] = b"_destroy_symbol";

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

    pub unsafe fn load(&mut self, path: String) -> Result<&mut Box<Plugin>, String> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        Self::log(&format!("Loading Plugin: {}", path));

        let lib = Library::new(path);
        match lib {
            Ok(l) => {
                self.loaded_libraries.push(l);
                let lib = self.loaded_libraries.last().unwrap();
                match lib.get(_PLUGIN_CREATE_SYMBOL) {
                    Ok(sym) => {
                        let func = sym as Symbol<PluginCreate>;
                        let pg_raw = func();
                        let mut pg = Box::from_raw(pg_raw);
                        // Yes, I know this isn't technically good rust, but we're in
                        // unsafe code anyway. I'd prefer to keep an immutable reference
                        // to all plugins loaded so we can control memory rather than let
                        // the references go loose into the rest of the code.
                        self.loaded_plugins.push(pg);
                        let pg = self.loaded_plugins.last_mut().unwrap();
                        Ok(pg)
                    }
                    Err(_e) => Err(format!("Unable to load _create symbol: {}", _e).to_string()),
                }
            }
            Err(_e) => Err(format!("Unable to load library file: {}", _e).to_string()),
        }
    }
}
