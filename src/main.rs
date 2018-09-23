#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tesys_derive;

mod tesys;
use tesys::astrometry::Angle;
use tesys::Peer;

fn main() {
    tesys::loggable::log("Starting Tesys...");
    let mut _an: Angle = Angle::new(12.0);
    let _hms = _an.to_hms();
    println!("{}", _hms);
    let _an2: Angle = Angle::new_from_hms(_hms);

    tesys::loggable::log("Initialising Peer...");
    let _p = Peer::new();
}
