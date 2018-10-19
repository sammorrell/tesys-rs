use dms;
use hms;
use std::clone::Clone;
use std::fmt;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use DEG_PER_RAD;

pub struct Angle {
    _angle: f32, // Measured in radians
}

impl Angle {
    pub fn new(val: f32) -> Angle {
        // Creates a new instance of the object given an angle in degrees.
        Angle {
            _angle: val / DEG_PER_RAD,
        }
    }

    pub fn new_from_hms(val: hms::HMS) -> Angle {
        // Creates a new instance of the object given an angle in degrees.
        Angle { _angle: val.into() }
    }

    pub fn set(&mut self, val: f32) {
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

    pub fn sin(&self) -> f32 {
        self._angle.sin()
    }

    pub fn cos(&self) -> f32 {
        self._angle.cos()
    }

    pub fn tan(&self) -> f32 {
        self._angle.tan()
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

impl From<i8> for Angle {
    fn from(val: i8) -> Self {
        Angle::new(val as f32)
    }
}

impl From<i32> for Angle {
    fn from(val: i32) -> Self {
        Angle::new(val as f32)
    }
}

impl From<f32> for Angle {
    fn from(val: f32) -> Self {
        Angle::new(val)
    }
}

/* Operations */
impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        self._angle = self._angle + rhs._angle;
    }
}

impl AddAssign<f32> for Angle {
    fn add_assign(&mut self, rhs: f32) {
        self._angle = self._angle + rhs;
    }
}

impl AddAssign<i32> for Angle {
    fn add_assign(&mut self, rhs: i32) {
        self._angle = self._angle + (rhs as f32);
    }
}

impl SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        self._angle = self._angle - rhs._angle;
    }
}

impl SubAssign<f32> for Angle {
    fn sub_assign(&mut self, rhs: f32) {
        self._angle = self._angle - rhs;
    }
}

impl SubAssign<i32> for Angle {
    fn sub_assign(&mut self, rhs: i32) {
        self._angle = self._angle - (rhs as f32);
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Angle {
        Angle {
            _angle: self._angle + rhs._angle,
        }
    }
}

impl Add<f32> for Angle {
    type Output = Angle;

    fn add(self, rhs: f32) -> Angle {
        Angle {
            _angle: self._angle + rhs,
        }
    }
}

impl Add<i32> for Angle {
    type Output = Angle;

    fn add(self, rhs: i32) -> Angle {
        Angle {
            _angle: self._angle + (rhs as f32),
        }
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Angle {
        Angle {
            _angle: self._angle - rhs._angle,
        }
    }
}

impl Sub<f32> for Angle {
    type Output = Angle;

    fn sub(self, rhs: f32) -> Angle {
        Angle {
            _angle: self._angle - rhs,
        }
    }
}

impl Sub<i32> for Angle {
    type Output = Angle;

    fn sub(self, rhs: i32) -> Angle {
        Angle {
            _angle: self._angle - (rhs as f32),
        }
    }
}

impl Clone for Angle {
    fn clone(&self) -> Angle {
        Angle {
            _angle: self._angle,
        }
    }
}
