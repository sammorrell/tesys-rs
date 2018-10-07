use loggable::*;

extern crate libloading as lib;

pub trait Plugin {
	fn new() -> Self where Self: Sized;
	fn test(self: &Self);
}

lazy_static! {
	pub static ref LOADED_LIBRARIES: Vec<lib::Library> = Vec::new();
}


pub fn instance_from_path<'a>(path: String) -> Result<Box<Plugin>, ()> {
	log(&format!("Loading Plugin: {}", path));

	let pglib = lib::Library::new(path).unwrap();

	unsafe {
		let func: lib::Symbol<unsafe extern fn() -> Box<Plugin>> = pglib.get(b"create").unwrap();
		let pg = func();
		Ok(pg)
	}
}