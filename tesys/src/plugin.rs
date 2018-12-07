use std::any::Any;
use std::fmt::Debug;
pub use crate::Exchange;

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

// Inspired by: https://stackoverflow.com/questions/45232838/is-it-possible-to-automatically-define-fields-of-a-struct
#[macro_export]
macro_rules! tesys_plugin {
	($struct:ident { $( $field:ident:$type:ty, )* }) => {
		#[derive(Debug,Clone,Loggable,Routable)]
        pub struct $struct {
            test_field: String,
            _exchange: tesys::Exchange,
            $(
                $field: $type,
            )*
        }
    };
}

#[macro_export]
macro_rules! tesys_plugin_new {
	($( $field:ident:$value:expr, )*) => {
		fn new() -> Self {
			Self {
				test_field: "Test".to_owned(),
				_exchange: tesys::Exchange::new(),
				$(
	                $field: $value,
	            )*
			}
		}
	}
}

pub trait Plugin: Any + Send + Sync + Debug {
    fn new() -> Self
    where
        Self: Sized;
    fn test(self: &mut Self);
}
