use crate::net::router::{Inlet, Outlet};
use crate::net::Message;

pub trait Routable {
    fn set_inlet(&mut self, inlet: Inlet);
    fn set_outlet(&mut self, inlet: Outlet);
    fn get_handle(&self) -> String;
}
