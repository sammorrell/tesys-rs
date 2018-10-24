use chrono::{DateTime, Local};
use Angle;
use Location;

const SECONDS_PER_SIDEREAL_DAY: f64 = 24.0 * 3600.0; // The number of seconds in 24 hours, given in f64. 
const SECONDS_PER_SIDEREAL_YEAR: f64 = SECONDS_PER_SIDEREAL_DAY * 365.25636; // The number of seconds in a sidereal year, given in f64. 
const MJD_OFFSET: f64 = 2400000.5; // The offset between Julian Date and Mdofied Julian Date, represented in Julian days. Given in f64.
const UNIX_EPOCH: f64 = 2440587.5; // The time of the UNIX Epoch (1970-01-01 12:00:00) in Julian days. 

pub fn datetime_to_julian_date(_dt: DateTime<Local>) -> f64 {
    //! Converts a chrono::DateTime to a Julian Date in f64 format. 
    UNIX_EPOCH + ( _dt.timestamp() as f64 / SECONDS_PER_SIDEREAL_DAY )
}

pub fn datetime_to_modified_julian_date(_dt: DateTime<Local>) -> f64 {
	//! Converts a chrono::DateTime to a Julian Date in f64 format.
    datetime_to_julian_date(_dt) - MJD_OFFSET
}

pub fn datetime_to_gmst(_dt: DateTime<Local>) -> Angle {
	let jd = datetime_to_julian_date(_dt);
	let t = (jd - 2451545.0) / (36525.0);
	let mut st = 280.46061837 + 360.98564736629 * ( jd - 2451545.0 );
	st += 0.000387933 * t.powi(2) - t.powi(3) / 38710000.0;
	st = st % 360.;

	Angle::new(st)
}

pub fn get_sidereal_time(_dt: DateTime<Local>, _loc: Location) -> Angle {
	let lon: f64 = if _loc.lon.deg() < 0.0 { -360.0 - (_loc.lon.deg() % 360.0 as f64) } else { _loc.lon.deg() };
	let lmst: f64 = ((datetime_to_gmst(_dt).deg() - lon) % 360.).into();

	Angle::new(lmst)
}
