use std::fmt;
use DEG_PER_RAD;

pub struct DMS {
    pub d: i8,
    pub m: i8,
    pub s: f32,
    pub neg: bool,
}

impl DMS {
    /*!
     * A container that can be used to express angles as hours, minutes and seconds.
     * The hours and minutes are represented as i8, whereas seconds has a fractional part
     * thus is represented by an f32.
     */

    pub fn new(_d: i8, _m: i8, _s: f32, _neg: bool) -> DMS {
        DMS {
            d: _d,
            m: _m,
            s: _s,
            neg: _neg,
        }
    }

    pub fn new_from_rad(&rad: &f32) -> DMS {
        /*!
         * Creates a new instance of DMS object from a radian measurement, given as an f32.
         * As the Angle struct uses an f32 representation of radians internally, this is what
         * gets called when you call the ```Angle::to_dms()``` function.
         */

        let _neg = if rad < 0. { true } else { false };
        let dummy = rad * DEG_PER_RAD * {
            if _neg {
                -1.0
            } else {
                1.0
            }
        };
        let _h: i8 = dummy.floor() as i8;
        let _m: i8 = (dummy.fract() * 60.0).floor() as i8;
        let _s: f32 = (dummy.fract() * 60.0).fract() * 60.;

        DMS {
            d: _h,
            m: _m,
            s: _s,
            neg: _neg,
        }
    }

    pub fn new_from_str(_dms_str: String) -> DMS {
        //! Generates a new HMS struct from a String formatted as
        //! *hh:mm:ss.s*. Also accepts negative sign.
        let str_vec = _dms_str.split(":");
        for d in str_vec {
            println!("{}", d);
        }

        let _h: i8 = 0;
        let _m: i8 = 0;
        let _s: f32 = 0.0;
        let _sign: bool = false;
        DMS::new(_h, _m, _s, _sign)
    }
}

impl fmt::Display for DMS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.neg { "-" } else { "" };
        write!(f, "{}{:0>2}:{:0>2}:{:2.2}", sign, self.d, self.m, self.s)
    }
}

impl Into<f32> for DMS {
    fn into(self) -> f32 {
        (self.d as f32 + (self.m as f32) / 60.0 + (self.s as f32) / 3600.0) / DEG_PER_RAD
    }
}
