use std;
use hms;
use wrap_angle;

pub struct Angle {
	_angle: f32,
}

impl Angle {

	pub fn new(val: f32) -> Angle {
		// Creates a new instance of the object given an angle in degrees.
		Angle {
			_angle: val
		}
	}

	pub fn new_from_hms(val: hms::HMS) -> Angle {
		// Creates a new instance of the object given an angle in degrees.
		Angle {
			_angle: val.into()
		}
	}

	pub fn set(&mut self, val: f32 ) {
		self._angle = val;
	}

	pub fn to_hms(&mut self) -> hms::HMS {
		let hms = hms::HMS::new_from_rad(self._angle);
		return hms;
	}
}