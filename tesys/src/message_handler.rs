use crate::Routable;
use crate::Message;
use std::any::Any;
use std::fmt::Debug;

pub trait MessageHandler: Any + Send + Sync {
	fn can_handle(&self, handle: String) -> bool;
    fn handle(&mut self, handle: String, m: Message) -> Option<Message>;
}