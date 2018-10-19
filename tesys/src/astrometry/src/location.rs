use Angle;

pub struct Location {
    lat: Angle,
    lon: Angle,
}

impl Location {
    pub fn new(_lat: f32, _lon: f32) -> Location {
        Location {
            lat: Angle::new(_lat),
            lon: Angle::new(_lon),
        }
    }
}
