use crate::frame::Frame;
use crate::Angle;
use crate::Epoch;
use crate::ProperMotion;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct SkyCoordinate<F: Frame> {
    pub coords: Vec<Angle>,
    pub _pm: ProperMotion,
    pub _epoch: Epoch,
    pub _frame: F,
}

impl<F: Frame> SkyCoordinate<F> {}
