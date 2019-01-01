use crate::router::{Inlet, Outlet};
use crate::Message;

pub trait Routable {
    fn set_inlet(&mut self, inlet: Inlet);
    fn set_outlet(&mut self, inlet: Outlet);
    fn get_handle(&self) -> String;
}
