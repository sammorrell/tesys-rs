use chrono::{DateTime, Local};
use Angle;
use AzEl;
use Location;

use std::fmt;

pub struct SkyCoordinate {
    pub ra: Angle,
    pub dec: Angle,
}

impl SkyCoordinate {
    pub fn new(_ra: f32, _dec: f32) -> SkyCoordinate {
        let ra = Angle::new(_ra);
        let dec = Angle::new(_dec);
        SkyCoordinate { ra: ra, dec: dec }
    }

    pub fn to_current_sky_position(&self, _loc: Location) -> AzEl {
        let dt = Local::now();
        self.to_sky_position(_loc, dt)
    }

    pub fn to_sky_position(&self, _loc: Location, _dt: DateTime<Local>) -> AzEl {
        AzEl {
            az: Angle::new(0.),
            el: Angle::new(0.),
        }
    }
}

impl fmt::Display for SkyCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ra_hms = self.ra.to_hms();
        let dec_dms = self.dec.to_dms();
        let ra_neg_str = if ra_hms.neg { "-" } else { "" };
        let dec_neg_str = if dec_dms.neg { "-" } else { "" };
        write!(
            f,
            "RA = {}{:0>2}:{:0>2}:{:2.2}, DEC = {}{:0>2}:{:0>2}:{:2.2}",
            ra_neg_str, ra_hms.h, ra_hms.m, ra_hms.s, dec_neg_str, dec_dms.d, dec_dms.m, dec_dms.s
        )
    }
}
