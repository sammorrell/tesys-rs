use Angle;

pub struct Location {
	lat: Angle, 
	lon: Angle,
}

impl Location {
	pub fn new(lat: f32, lon: f32) -> Location {
		Location {
			lat: Angle::new(lat),
			lon: Angle::new(lon),
		}
	}
}