use crate::Angle;
use crate::ProperMotion;
use crate::Epoch;
use std::vec::Vec;
use crate::frame::Frame;

#[derive(Clone, Debug)]
pub struct SkyCoordinate<F: Frame>  {
    pub coords: Vec<Angle>,
    pub _pm: ProperMotion,
    pub _epoch: Epoch,
    pub _frame: F,
}

impl<F: Frame> SkyCoordinate<F> {
}
