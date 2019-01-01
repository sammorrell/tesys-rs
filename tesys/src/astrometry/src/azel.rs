use crate::angle::Angle;
use std::fmt;

pub struct AzEl {
    pub az: Angle,
    pub el: Angle,
}

impl AzEl {
    pub fn new(az: f64, el: f64) -> AzEl {
        AzEl {
            az: Angle::new(az),
            el: Angle::new(el),
        }
    }
}

impl fmt::Display for AzEl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AZ = {:0>2}, EL = {:0>2}", self.az, self.el)
    }
}
