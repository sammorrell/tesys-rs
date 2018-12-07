use crate::Location;
use crate::SkyCoordinate;
use crate::frame::Frame;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct CoordinateTransform<F: Frame, T: Frame> {
	pub coords: SkyCoordinate<F>,
	pub from: F,
	pub to: T,
	pub at: DateTime<Utc>,
	pub loc: Location,
}

impl<F: Frame, T: Frame> CoordinateTransform<F, T> {
	pub fn new(coords: SkyCoordinate<F>) -> CoordinateTransform<F, T> {
		CoordinateTransform {
			coords: coords,
			from: F::new(), 
			to: T::new(),
			at: Utc::now(),
			loc: Location::new(0.0, 0.0)
		}
	}

	pub fn at(self, _dt: DateTime<Utc>) -> CoordinateTransform<F, T> {
		let mut ret = self.clone();
		ret.at = _dt;
		ret
	}

	pub fn with_location(self, _loc: Location) -> CoordinateTransform<F, T> {
		let mut ret = self.clone();
		ret.loc = _loc;
		ret
	}
}