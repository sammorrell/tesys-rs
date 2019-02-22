// Let's include the Celestial Intermediate Reference System. 
pub mod cirs;
pub use self::cirs::CIRS;

pub mod fk5;
pub use self::fk5::FK5;
pub mod icrs;
pub use self::icrs::ICRS;
pub mod horizontal;
pub use self::horizontal::Horizontal;
pub mod galactic;
pub use self::galactic::Galactic;
