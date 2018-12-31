use std::fmt::Debug;
use std::any::Any;

pub trait Payload: Any + Send + Sync + Debug {
	
}