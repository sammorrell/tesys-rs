use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Clone, Debug)]
pub struct Exchange {}

impl Exchange {
    pub fn new() -> Exchange {
        Exchange {}
    }
}
