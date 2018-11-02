use chrono::{DateTime, Local};
use Angle;
use AzEl;
use Location;

use std::fmt;
use std::f64;
use datetime::*;

pub struct SkyCoordinate {
    pub ra: Angle,
    pub dec: Angle,
}

impl SkyCoordinate {
    pub fn new(_ra: f64, _dec: f64) -> SkyCoordinate {
        let ra = Angle::new(_ra);
        let dec = Angle::new(_dec);
        SkyCoordinate { ra: ra, dec: dec }
    }

    pub fn to_current_sky_position(&self, _loc: Location) -> AzEl {
        let dt = Local::now();
        self.to_sky_position(dt, _loc)
    }

    pub fn to_sky_position(&self, _dt: DateTime<Local>, _loc: Location) -> AzEl {
        let ra = self.ra.clone();
        let mut ha = get_sidereal_time(_dt, _loc.clone()) - ra;
        ha = if ha < Angle::new(0.0) { ha + 2.0 * f64::consts::PI } else { ha };
        ha = if ha > Angle::new(360.0) { ha - 2.0 * f64::consts::PI } else { ha };

        let az = Angle::new_from_atan( ha.sin() / (ha.cos() * _loc.lat.sin() - self.dec.tan() * _loc.lat.cos()) );
        let el = Angle::new_from_asin( _loc.lat.sin() * self.dec.sin() + _loc.lat.cos() * self.dec.cos() * ha.cos() );
        //let az = Angle::new(- ( - _loc.lat.sin() * self.dec.cos() * ha.cos()).atan2(self.dec.cos() * ha.sin() ));
        
        AzEl {
            az: az,
            el: el,
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
