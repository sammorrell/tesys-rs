extern crate chrono;

// Constants
const DEG_PER_RAD: f64 = { 180.0 / std::f64::consts::PI };
const HOUR_PER_DEG: f64 = { 1.0 / 15.0 };
const HOUR_PER_RAD: f64 = { HOUR_PER_DEG * DEG_PER_RAD };
const ARCSEC_PER_DEG: f64 = { 3600.0 };
const ARCSEC_PER_RAD: f64 = { ARCSEC_PER_DEG * DEG_PER_RAD };
const MAS_PER_DEG: f64 = { ARCSEC_PER_DEG * 1000.0 };
const MAS_PER_RAD: f64 = { ARCSEC_PER_RAD * 1000.0 };

const DEFAULT_WRAP_MAX_ANGLE: f64 = 2.0 * std::f64::consts::PI;
const DEFAULT_WRAP_MIN_ANGLE: f64 = 0.0;

pub mod epoch;
pub use epoch::Epoch;

pub mod datetime;

mod angle;
pub use angle::Angle;

mod hms;
pub use hms::HMS;

mod dms;
pub use dms::DMS;

mod azel;
pub use azel::AzEl;

mod location;
pub use location::Location;

mod propermotion;
pub use propermotion::ProperMotion;

pub mod frame;
pub mod frames;

mod skycoordinate;
pub use skycoordinate::SkyCoordinate;

pub fn wrap_angle(val: f64) -> f64 {
    // This is used to wrap the angle between the limits specified within the code.
    let mut rads = if val < DEFAULT_WRAP_MIN_ANGLE {
        DEFAULT_WRAP_MIN_ANGLE - (val % DEFAULT_WRAP_MIN_ANGLE)
    } else {
        val
    };
    rads %= DEFAULT_WRAP_MAX_ANGLE;
    rads
}
