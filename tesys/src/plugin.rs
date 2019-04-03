pub use crate::net::{CanHandleMessages, MessageHandler, Exchange};
use std::any::Any;
use std::fmt::Debug;

pub const PLUGIN_CREATE_SYMBOL: &[u8] = b"_create_plugin";
pub const PLUGIN_DESTROY_SYMBOL: &[u8] = b"_destroy_plugin";

pub type PluginCreate = unsafe fn() -> *mut Plugin;
pub type PluginDestroy = unsafe fn(*mut Plugin);

#[macro_export]
macro_rules! tesys_plugin_create {
    ($struct_name: ident) => {
        #[no_mangle]
        pub extern "C" fn _create_plugin() -> *mut Plugin {
            let obj = $struct_name::new();
            let boxed: Box<$struct_name> = Box::new(obj);
            Box::into_raw(boxed)
        }
    };
}

#[macro_export]
macro_rules! tesys_plugin_destroy {
    ($struct_name: ident) => {
        #[no_mangle]
        pub extern "C" fn _delete_plugin(p: *mut Plugin) {
            drop(p);
        }
    };
}

// Inspired by: https://stackoverflow.com/questions/45232838/is-it-possible-to-automatically-define-fields-of-a-struct
#[macro_export]
macro_rules! tesys_plugin {
	($struct:ident { $( $field:ident:$type:ty, )* }) => {
		#[derive(Debug,Clone,Loggable)]
        pub struct $struct {
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
				$(
	                $field: $value,
	            )*
			}
		}
	}
}

pub trait Plugin: Any + Send + Sync + Debug + CanHandleMessages + MessageHandler {
    fn new() -> Self
    where
        Self: Sized;
    fn test(self: &mut Self);

    /// Initialises your plugin so that it can begin running.
    fn init(&mut self);
    /// Terminates your plugin so it can be shut down.
    fn term(&mut self);
}
