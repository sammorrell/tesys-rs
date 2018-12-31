type StaticHandler<P,R> = fn(&mut P) -> Result<R, ()>;

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct StaticHandlerInfo<T> {
	pub name: &'static str,
	pub handler: fn(Box<T>),
}
