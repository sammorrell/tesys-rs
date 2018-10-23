use chrono::{DateTime, Local};


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

