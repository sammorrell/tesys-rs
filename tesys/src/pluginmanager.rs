extern crate libloading;
use self::libloading::{Library, Symbol};
use std::path::Path;

use loggable;
use loggable::Loggable;
use Plugin;

const _PLUGIN_CREATE_SYMBOL: &[u8] = b"_create_plugin";
const _PLUGIN_DESTROY_SYMBOL: &[u8] = b"_destroy_symbol";

#[derive(Loggable)]
pub struct PluginManager {
    loaded_plugins: Vec<Box<Plugin>>,
    loaded_libraries: Vec<Library>,

    // The vector containing plugin search paths
    plugin_search_paths: Vec<&'static str>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        let pl = PluginManager {
            loaded_plugins: Vec::new(),
            loaded_libraries: Vec::new(),
            plugin_search_paths: vec!("./"),
        };

        pl // Return our new plugin manager instance
    }

    pub fn load(&mut self, id: &'static str) -> Result<&mut Box<Plugin>, String> {
        match self._resolve_plugin_lib(id) {
            Ok(path) => unsafe{ self._load_plugin(path.to_string()) },
            Err(_e) => Err(format!("Unable to resolve library for plugin '{}'", id).to_string()),
        }
    }

    fn _resolve_plugin_lib(&self, lib: &'static str) -> Result<String, ()> {
        let plugin_search_paths = self.plugin_search_paths.clone();
        let mut base = String::new();
        let mut dir = String::new();

        for path in plugin_search_paths {
            if &lib[0..2] == "./" {
                // This is a relative path, we should resolve it within the current plugin dir
                let libtr = &lib[1..];

                if Path::new(libtr).exists() {
                    return Ok(libtr.to_owned());
                }  else {
                    tesys_err!(Self, "Unable to find plugin '{}' at relative path. ", lib);
                }
            } else if &lib[0..1] == "/" {
                // If it's absolute, we don't need to resolve, so just check if the file exists
                if Path::new(lib).exists() {
                    return Ok(lib.to_owned());
                } else {
                    tesys_err!(Self, "Unable to find plugin '{}' at absolute path. ", lib);
                }
            }  else {
                // Assume it's an relative path, so prepend the library dir
                base = lib.to_string();
                dir = path.to_owned();
            }

            //if Path::new( &format!("{}/{}", path.to_owned(), base) ).is_dir() { plugin_search_paths.push( &format!("{}/{}", path.to_owned(), base)) }
            
            // Append a forward slash if there isn't one
            if dir.chars().last() != Some('/') {
                dir.push_str("/");
            }

            // First let's check for a .so file on UNIX type systems
            if Path::new( &format!("{}lib{}.so", dir, base)).exists() {
                return Ok(format!("{}lib{}.so", dir, base).to_owned());
            }

            // Now we check for another favourite, dylib.
            if Path::new( &format!("{}lib{}.dylib", dir, base)).exists() {
                return Ok(format!("{}lib{}.dylib", dir, base).to_owned());
            }

            // Now let's check for a framework, and correct the path accordingly to actually get the lib
            if Path::new( &format!("{}lib{}.framework", dir, base)).is_dir() {
                return Ok(format!("{}lib{}.framework/{}", dir, base, base).to_owned());
            }

            if Path::new( &format!("{}lib{}.dll", dir, base)).is_dir() {
                return Ok(format!("{}lib{}.dll", dir, base).to_owned());
            }
        }

        tesys_err!("Unable to find resolve plugin '{}'.", lib);
        Err(())
    }

    unsafe fn _load_plugin(&mut self, path: String) -> Result<&mut Box<Plugin>, String> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        tesys_log!("Loading Plugin: {}", path);

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

    pub fn add_plugin_search_directory(&mut self, path: &'static str) {
        let tmp = path.clone();
        let fin: &'static str;
        let dir = Path::new(&tmp);

        if dir.exists() && dir.is_dir() {
            if &tmp[0..1] == "/" {
                fin = &tmp[1..];
            } else {
                fin = &tmp;
            }
            self.plugin_search_paths.push(fin);
        }
    }

    pub fn set_plugin_search_directories(&mut self, paths: Vec<&'static str>) {
        self.plugin_search_paths = paths;
    }
}
