pub trait Routable: Sized {
	fn recv(&mut self);
}