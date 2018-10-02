use HOUR_PER_RAD;
use std::fmt;

pub struct HMS {
	pub h: i8,
	pub m: i8,
	pub s: f32,
	pub neg: bool,
}

impl HMS {
	/*!
	 * A container that can be used to express angles as hours, minutes and seconds.
	 * The hours and minutes are represented as i8, whereas seconds has a fractional part
	 * thus is represented by an f32.
	 */

	pub fn new(_h: i8, _m: i8, _s: f32, _neg: bool) -> HMS {
		HMS {
			h: _h,
			m: _m,
			s: _s,
			neg: _neg,
		}
	}

	pub fn new_from_rad(&rad: &f32) -> HMS {
		/*!
		 * Creates a new instance of HMS object from a radian measurement, given as an f32.
		 * As the Angle struct uses an f32 representation of radians internally, this is what
		 * gets called when you call the ```Angle::to_hms()``` function. 
		 */

		let _neg = if rad < 0. { true } else  { false };
		let dummy = rad * HOUR_PER_RAD * { if _neg { -1.0 } else { 1.0 } };
		let _h: i8 = dummy.floor() as i8;
		let _m: i8 = (dummy.fract() * 60.0).floor() as i8;
		let _s: f32 = (dummy.fract() * 60.0).fract() * 60.;

		HMS {
			h: _h,
			m: _m,
			s: _s,
			neg: _neg,
		}
	}

	pub fn new_from_str(_hms_str: String) -> HMS {
		//! Generates a new HMS struct from a String formatted as
		//! *hh:mm:ss.s*. Also accepts negative sign.
		let str_vec = _hms_str.split(":");
		for d in str_vec {
			println!("{}", d);
		}

		let _h: i8 = 0;
		let _m: i8 = 0;
		let _s: f32 = 0.0;
		let _sign: bool = false;
		HMS::new(_h, _m, _s, _sign)
	}
}

impl fmt::Display for HMS {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let sign = if self.neg { "-" } else { "" };
        write!(f, "{}{:0>2}:{:0>2}:{:2.2}", sign, self.h, self.m, self.s)
    }
}

impl Into<f32> for HMS {
	fn into(self) -> f32 {
 		(self.h as f32 + (self.m as f32) / 60.0 + (self.s as f32) / 3600.0) / HOUR_PER_RAD
	}
}
