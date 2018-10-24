use angle::Angle;

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
