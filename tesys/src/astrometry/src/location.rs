use crate::Angle;

#[derive(Clone)]
pub struct Location {
    pub lat: Angle,
    pub lon: Angle,
}

impl Location {
    pub fn new(_lat: f64, _lon: f64) -> Location {
        Location {
            lat: Angle::new(_lat),
            lon: Angle::new(_lon),
        }
    }
}
