// Constants
const DEG_PER_RAD: f32 = 57.295779513;

const DEFAULT_WRAP_MAX_ANGLE: f32 = 360.0;
const DEFAULT_WRAP_MIN_ANGLE: f32 = 0.0;

mod angle;
pub use angle::Angle;

mod hms;
pub use hms::HMS;

pub fn wrap_angle(val: f32) -> f32 {
	// This is used to wrap the angle between the limits specified within the code. 
	
	let mut rads = val;
	rads %= 2.0 * std::f32::consts::PI;
	rads
}