use std::any::Any;
use std::fmt::Debug;

pub trait Payload: Any + Send + Sync + Debug + PayloadClone {
    fn pack<T>(obj: T) -> Box<T> where Self: Sized, T: Payload;
    fn unpack<T>(pl: Box<T>) -> Result<&'static mut T, ()> where Self: Sized, T: Payload;
}

impl<P: Any + Debug + Sync + Send + Clone + Sized> Payload for P {
    fn pack<T>(obj: T) -> Box<T> {
        Box::new(obj)
    }

    fn unpack<T>(pl: Box<T>) -> Result<&'static mut T, ()> {
        Ok(Box::leak(pl))
    }
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
