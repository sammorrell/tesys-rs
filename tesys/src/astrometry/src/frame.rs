/// This trait should be applied to structs that are to be used as the basis frame to a coordinates
/// system in the SkyCoordinate struct. 
///

pub trait Frame: Sized + Clone {
	type Frame;

	fn new() -> Self::Frame;
}

pub trait CanTransformTo<F> {
	type Output;
	fn transform(self) -> Self::Output;
}

// Import the reference frame stbibructs from the submodule.
pub use frames::icrs::ICRS;
pub use frames::fk5::FK5;