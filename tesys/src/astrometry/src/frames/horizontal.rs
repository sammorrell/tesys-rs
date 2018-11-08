use frame::Frame;

#[derive(Clone)]
pub struct Horizontal {
	
}

impl Frame for Horizontal {
	type Frame = Horizontal;

	fn new() -> Self::Frame {
		Horizontal {
			
		}
	}
}