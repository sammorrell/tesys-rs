extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::Plugin;
use tesys::loggable::*;

#[no_mangle]
pub fn create() -> Box<ExamplePlugin> {
	Box::new(ExamplePlugin::new())
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
		println!("Test");
	}
}