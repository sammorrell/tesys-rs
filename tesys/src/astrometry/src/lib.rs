// Constants
const DEG_PER_RAD: f32 = { 180.0 / std::f32::consts::PI };
const HOUR_PER_DEG: f32 = { 1.0 / 15.0 };
const HOUR_PER_RAD: f32 = { HOUR_PER_DEG * DEG_PER_RAD };

const DEFAULT_WRAP_MAX_ANGLE: f32 = 2.0 * std::f32::consts::PI;
const DEFAULT_WRAP_MIN_ANGLE: f32 = 0.0;

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

mod skycoordinate;
pub use skycoordinate::SkyCoordinate;

pub fn wrap_angle(val: f32) -> f32 {
    // This is used to wrap the angle between the limits specified within the code.
    let mut rads = if val < DEFAULT_WRAP_MIN_ANGLE {
        DEFAULT_WRAP_MIN_ANGLE - (val % DEFAULT_WRAP_MIN_ANGLE)
    } else {
        val
    };
    rads %= DEFAULT_WRAP_MAX_ANGLE;
    rads
}
