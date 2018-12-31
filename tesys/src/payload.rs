use std::fmt::Debug;
use std::any::Any;

pub trait Payload: Any + Send + Sync + Debug + PayloadClone {
	
}

pub trait PayloadClone {
    fn clone_payload(&self) -> Box<Payload>;
}

impl<T> PayloadClone for T
where
    T: 'static + Payload + Clone,
{
    fn clone_payload(&self) -> Box<Payload> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Payload> {
    fn clone(&self) -> Self {
        self.clone_payload()
    } 
}