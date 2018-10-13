use loggable::*;
use std::any::Any;

pub trait Plugin: Any + Send + Sync {
	fn new() -> Self where Self: Sized;
	fn test(self: &mut Self);
}