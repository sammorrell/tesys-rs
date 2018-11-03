use chrono::{DateTime, Local, Utc};
use Angle;
use AzEl;
use Location;
use ProperMotion;
use Epoch;

use std::fmt;
use std::f64;
use datetime::*;

pub struct SkyCoordinate {
    pub ra: Angle,
    pub dec: Angle,
    _pm: ProperMotion,
    _epoch: Epoch,
}

impl SkyCoordinate {
    pub fn new(_ra: f64, _dec: f64) -> SkyCoordinate {
        let ra = Angle::new(_ra);
        let dec = Angle::new(_dec);
        SkyCoordinate { ra: ra, dec: dec, _pm: ProperMotion::new(0., 0.), _epoch: Epoch::now() }
    }

    pub fn with_epoch(mut self, _epoch: Epoch) -> Self {
        self._epoch = _epoch;
        self
    }

    pub fn with_proper_motion(mut self, _pm: ProperMotion) -> Self {
        self._pm = _pm;
        self
    }

    pub fn to_current_sky_position(&self, _loc: Location) -> AzEl {
        let dt = Local::now();
        self.to_sky_position(dt, _loc)
    }

    pub fn to_epoch(&self, _ep: Epoch) -> SkyCoordinate {
        let ra = self.ra.clone();
        let dec = self.dec.clone();
        let dtyr = - ((self._epoch.clone() - _ep.clone()).num_milliseconds() as f64 / 1000.0) / SECONDS_PER_SIDEREAL_YEAR;
        SkyCoordinate {
            ra: ra + (self._pm.pmra.clone() * dtyr),
            dec: dec + (self._pm.pmdec.clone() * dtyr),
            _pm: self._pm.clone(),
            _epoch: _ep
        }
    }

    pub fn to_sky_position(&self, _dt: DateTime<Local>, _loc: Location) -> AzEl {
        let epoch = Epoch::new(_dt.with_timezone(&Utc));
        let tmpcoord = self.to_epoch(epoch);
        let mut ha = get_sidereal_time(_dt.with_timezone(&Utc), _loc.clone()) - tmpcoord.ra;
        ha = if ha < Angle::new(0.0) { ha + 2.0 * f64::consts::PI } else { ha };
        ha = if ha > Angle::new(360.0) { ha - 2.0 * f64::consts::PI } else { ha };
        println!("HA: {}", ha.to_hms());

        let az = Angle::new_from_atan( ha.sin() / (ha.cos() * _loc.lat.sin() - tmpcoord.dec.tan() * _loc.lat.cos()) );
        let el = Angle::new_from_asin( _loc.lat.sin() * tmpcoord.dec.sin() + _loc.lat.cos() * tmpcoord.dec.cos() * ha.cos() );
        //let az = Angle::new(- ( - _loc.lat.sin() * tmpcoord.dec.cos() * ha.cos()).atan2(tmpcoord.dec.cos() * ha.sin() ));
        
        AzEl {
            az: az,
            el: el,
        }
    }

    pub fn epoch(&self) -> Epoch {
        self._epoch.clone()
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
            "RA ({}) = {}{:0>2}:{:0>2}:{:2.2}, DEC ({}) = {}{:0>2}:{:0>2}:{:2.2}",
            self.epoch(), ra_neg_str, ra_hms.h, ra_hms.m, ra_hms.s, self.epoch(), dec_neg_str, dec_dms.d, dec_dms.m, dec_dms.s
        )
    }
}
