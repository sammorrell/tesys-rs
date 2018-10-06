use loggable::*;

extern crate libloading as lib;

pub trait PluginBase {
	fn new() -> Plugin;
}

pub struct Plugin {
}

impl Plugin {
	pub fn instance_from_path(path: String) -> Result<Box<Plugin>, ()> {
		log(&format!("Loading Plugin: {}", path));
		let pglib = lib::Library::new(path).unwrap();
		unsafe {
			let func: lib::Symbol<unsafe extern fn() -> Box<Plugin>> = pglib.get(b"create").unwrap();
			let pg = func();
			Ok(pg)
		}
	}
}

impl PluginBase for Plugin {
	fn new() -> Plugin {
		Plugin {
		}
	}
}