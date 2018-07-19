pub struct Angle {
	_angle: f32,
}

impl Angle {

	pub fn new(val: f32) -> Angle {
		Angle {
			_angle: val
		}
	}

	pub fn set(&mut self, val: f32 ) {
		self._angle = val;
	}
}