use std::any::Any;
use std::fmt::Debug;

pub trait Payload: Any + Send + Debug + PayloadClone {
    fn pack<T>(obj: T) -> Box<T> where Self: Sized, T: Payload + Clone;
    fn unpack<T>(pl: Box<dyn Payload>) -> Result<T, ()> where Self: Sized, T: Payload + Clone;
    fn as_any(&self) -> &dyn Any;
}

impl<P: Any + Debug + Sync + Send + Clone + Sized> Payload for P {
    fn pack<T: Clone>(obj: T) -> Box<T> {
        Box::new(obj)
    }

    fn unpack<T: Clone + 'static>(pl: Box<dyn Payload>) -> Result<T, ()> {
        match pl.as_any().downcast_ref::<T>() {
            Some(p) => Ok(p.clone()),
            None => Err(()),
        }
    }

     fn as_any(&self) -> &dyn Any {
        self
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
