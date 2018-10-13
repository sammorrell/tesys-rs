extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::Plugin;
use tesys::loggable::*;
use tesys::astrometry::{SkyCoordinate};

#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut Plugin {
	let obj = ExamplePlugin::new();
	let boxed: Box<ExamplePlugin> = Box::new(obj);
	Box::into_raw(boxed)
}

#[allow(dead_code)]
#[derive(Loggable)]
pub struct ExamplePlugin {
	label: String,
	coord: SkyCoordinate,
}

impl ExamplePlugin {

}

impl Plugin for ExamplePlugin {
	fn new() -> ExamplePlugin {
		ExamplePlugin::log("Loading Plugin");
		ExamplePlugin {
			label: "".to_string(),
			coord: SkyCoordinate::new(0.0, 0.0)
		}
	}

	fn test(&mut self) {
		Self::warn(&format!("{}", self.coord));
		self.coord.ra.set(23.0);
		self.coord.dec.set(14.0);
		Self::warn(&format!("{}", self.coord));
	}
}