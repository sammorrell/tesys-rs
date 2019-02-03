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
pub use crate::epoch::Epoch;

pub mod datetime;

mod angle;
pub use crate::angle::Angle;

mod hms;
pub use crate::hms::HMS;

mod dms;
pub use crate::dms::DMS;

mod azel;
pub use crate::azel::AzEl;

mod location;
pub use crate::location::Location;

mod propermotion;
pub use crate::propermotion::ProperMotion;

mod transform;
pub use crate::transform::CoordinateTransform;

pub mod frame;
pub use crate::frame::Frame;

pub mod frames;

mod skycoordinate;
pub use crate::skycoordinate::SkyCoordinate;

// Use the module for unit tests if we want it. 
#[cfg(test)]
mod tests;