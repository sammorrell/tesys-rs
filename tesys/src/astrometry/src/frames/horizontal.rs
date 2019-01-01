use crate::frame::Frame;

#[derive(Clone, Debug)]
pub struct Horizontal {}

impl Frame for Horizontal {
    fn new() -> Self {
        Horizontal {}
    }
}
