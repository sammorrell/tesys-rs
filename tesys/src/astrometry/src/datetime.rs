use chrono::{DateTime, Local};

const MJD_OFFSET: f64 = 2400000.5;

/**! Converts a chrono::DateTime to a Julian Date in f64 format. **/
pub fn datetime_to_julian_date(_dt: DateTime<Local>) -> f64 {
    0 as f64
}

/**! Converts a chrono::DateTime to a Julian Date in f64 format. **/
pub fn datetime_to_modified_julain_date(_dt: DateTime<Local>) -> f64 {
    datetime_to_julian_date(_dt) - MJD_OFFSET
}
