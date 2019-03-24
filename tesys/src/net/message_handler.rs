use crate::net::Message;
use crate::codegen::*;
use std::any::Any;

pub trait CanHandleMessages: Any {
    fn can_handle(&self, handle: String) -> bool;
    fn handle(&mut self, handle: String, m: Message) -> Option<Message>;
}

pub trait MessageHandler {
    fn get_handlers(&self) -> &HandlerTable<Self> where Self: Sized;
}
