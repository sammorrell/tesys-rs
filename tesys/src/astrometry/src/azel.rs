pub struct AzEl {
	az: Angle,
	el: Angle,
}

impl AzEl {
	pub fn new (az: f32, el: f32) {
		Angle {
			az: Angle::new(az),
			el: Angle::new(el),
		}
	}
}