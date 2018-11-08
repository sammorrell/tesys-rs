use frame::Frame;

#[derive(Clone)]
pub struct Galactic {
	
}

impl Frame for Galactic {
	type Frame = Galactic;

	fn new() -> Self::Frame {
		Galactic {
			
		}
	}
}