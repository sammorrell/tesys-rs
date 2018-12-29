use crate::Plugin;

pub type StaticHandler = fn(&mut Box<Plugin>);

#[derive(Clone,Debug)]
pub struct StaticHandlerInfo<T> {
	pub name: &'static str,
	pub handler: fn(Box<T>),
}
