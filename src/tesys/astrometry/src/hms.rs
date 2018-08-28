use DEG_PER_RAD;
use wrap_angle;
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
	 */

	pub fn new(_h: i8, _m: i8, _s: f32, _neg: bool) -> HMS {
		HMS {
			h: _h,
			m: _m,
			s: _s,
			neg: _neg,
		}
	}

	pub fn new_from_rad(_rad: f32) -> HMS {

		let _neg = if _rad < 0. { true } else  { false };
		let mut tmp = if _rad < 0. { -1.0 * _rad * DEG_PER_RAD } else { _rad * DEG_PER_RAD };
		let mut dummy = wrap_angle(tmp);
		let _h: i8 = 0;
		let _m: i8 = 0;
		let _s: f32 = 0.0;

		HMS {
			h: _h,
			m: _m,
			s: _s,
			neg: _neg,
		}
	}

	pub fn new_from_str(_hms_str: String) -> HMS {

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
        write!(f, "{}{}:{}:{:.2}", sign, self.h, self.m, self.s)
    }
}

impl Into<f32> for HMS {
	fn into(self) -> f32 {
 		0.0
	}
}
