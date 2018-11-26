use std::any::Any;
use std::fmt::Debug;

#[macro_export]
macro_rules! tesys_plugin_create {
	($struct_name: ident) => (
		#[no_mangle]
		pub extern "C" fn _create_plugin() -> *mut Plugin {
		    let obj = $struct_name::new();
		    let boxed: Box<$struct_name> = Box::new(obj);
		    Box::into_raw(boxed)
		}
	)
}

#[macro_export]
macro_rules! tesys_plugin_destroy {
	($struct_name: ident) => (
		#[no_mangle]
		pub extern "C" fn _delete_plugin(p: *mut Plugin) {
		    drop(p);
		}
	)
}

pub trait Plugin: Any + Send + Sync + Debug {
    fn new() -> Self
    where
        Self: Sized;
    fn test(self: &mut Self);
}
