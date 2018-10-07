extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::Plugin;
use tesys::loggable::*;

#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut Plugin {
	let obj = ExamplePlugin::new();
	let boxed: Box<ExamplePlugin> = Box::new(obj);
	Box::into_raw(boxed)
}

#[allow(dead_code)]
#[derive(Loggable)]
pub struct ExamplePlugin {
	label: String
}

impl ExamplePlugin {

}

impl Plugin for ExamplePlugin {
	fn new() -> ExamplePlugin {
		ExamplePlugin::log("Loading Plugin");
		ExamplePlugin {
			label: "".to_string(),
		}
	}

	fn test(&self) {
		Self::warn("Test");
	}
}