use crate::Message;
use std::any::Any;

pub trait CanHandleMessages: Any {
	fn can_handle(&self, handle: String) -> bool;
    fn handle(&mut self, handle: String, m: Message) -> Option<Message>;
}

pub struct MessageHandler {
    
}