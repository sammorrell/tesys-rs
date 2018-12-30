use std::collections::HashMap;

use crate::Plugin;

type StaticHandler<P,R> = fn(&mut P) -> Result<R, ()>;

#[derive(Clone)]
pub struct PluginVTable<P> {
	pub _vtbl: HashMap<String, StaticHandler<P, ()>>,
}

#[derive(Clone,Debug)]
pub struct StaticHandlerInfo<T> {
	pub name: &'static str,
	pub handler: fn(Box<T>),
}
