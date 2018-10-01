use hms;
use dms;
use std::fmt;
use std::clone::Clone;

use DEG_PER_RAD;

pub struct Angle {
	_angle: f32, // Measured in radians
}

impl Angle {

	pub fn new(val: f32) -> Angle {
		// Creates a new instance of the object given an angle in degrees.
		Angle {
			_angle: val / DEG_PER_RAD
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

	pub fn to_hms(&self) -> hms::HMS {
		let hms = hms::HMS::new_from_rad(&self._angle);
		return hms;
	}

	pub fn to_dms(&self) -> dms::DMS {
		let dms = dms::DMS::new_from_rad(&self._angle);
		return dms;
	}
}

impl fmt::Debug for Angle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:2.2}°", self._angle)
    }
}

impl fmt::Display for Angle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:2.2}°", self._angle * DEG_PER_RAD)
    }
}

impl Into<hms::HMS> for Angle {
	fn into(self) -> hms::HMS {
		return self.to_hms();
	}
}

impl Into<dms::DMS> for Angle {
	fn into(self) -> dms::DMS {
		return self.to_dms();
	}
}

impl Into<f32> for Angle {
	fn into(self) -> f32 {
 		self._angle
	}
}

impl Into<i8> for Angle {
	fn into(self) -> i8 {
		self._angle as i8
	}
}

impl Clone for Angle {
	fn clone(&self) -> Angle {
		Angle {
			_angle : self._angle
		}
	}
}
