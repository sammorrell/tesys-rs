use Angle;
use ProperMotion;
use Epoch;
use std::vec::Vec;
use frame::Frame;

#[derive(Clone)]
pub struct SkyCoordinate<F: Frame>  {
    pub coords: Vec<Angle>,
    pub _pm: ProperMotion,
    pub _epoch: Epoch,
    pub _frame: F,
}

