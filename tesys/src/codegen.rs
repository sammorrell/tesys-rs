pub type StaticHandler<P, R> = fn(&mut P) -> Result<R, ()>;

pub type HandlerTable<T> = Vec<&'static StaticHandlerInfo<T>>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct StaticHandlerInfo<T> {
    pub name: &'static str,
    pub handler: fn(Box<T>),
}
