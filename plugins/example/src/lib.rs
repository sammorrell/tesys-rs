extern crate tesys;
use tesys::Plugin;

#[no_mangle]
pub fn create() -> Box<Plugin> {
	println!("Loading Plugin");
	Box::new(ExamplePlugin::new())
}

#[allow(dead_code)]
pub struct ExamplePlugin {
	label: String
}

impl ExamplePlugin {

}

impl PluginBase for ExamplePlugin {
	pub fn new() -> ExamplePlugin {
		ExamplePlugin {
			label: "".to_string(),
		}
	}
}