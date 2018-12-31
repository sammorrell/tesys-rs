use crate::{Route,Payload};

pub struct Message {
	to: Route,
	from: Route,
	payload: Option<Box<Payload>>,
}

