use angle::Angle;

pub struct AzEl {
    pub az: Angle,
    pub el: Angle,
}

impl AzEl {
    pub fn new(az: f32, el: f32) -> AzEl {
        AzEl {
            az: Angle::new(az),
            el: Angle::new(el),
        }
    }
}
