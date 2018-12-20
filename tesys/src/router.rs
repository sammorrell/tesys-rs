extern crate slotmap;
// use slotmap::{SlotMap, DefaultKey};
// use std::sync::mpsc::{channel, Receiver, Sender};

use crate::Routable;

pub struct Router {
}

impl Router {
	pub fn new() -> Router {
		Router {}
	}

	pub fn register<T: Routable>(_item: T) {

	}
}