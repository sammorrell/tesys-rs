use crate::frame::Frame;
use crate::Location;
use crate::SkyCoordinate;
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
    pub fn new<'a>(coords: SkyCoordinate<F>) -> CoordinateTransform<F, T> {
        CoordinateTransform {
            coords: coords,
            from: F::new(),
            to: T::new(),
            at: Utc::now(),
            loc: Location::new(0.0, 0.0),
        }
    }

    pub fn at<'a>(&'a mut self, _dt: DateTime<Utc>) -> &'a mut CoordinateTransform<F, T> {
        self.at = _dt;
        self
    }

    pub fn with_location<'a>(&'a mut self, _loc: Location) -> &'a mut CoordinateTransform<F, T> {
        self.loc = _loc;
        self
    }
}
