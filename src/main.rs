#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tesys_derive;

mod tesys;
use tesys::astrometry::{Angle, HMS};
use tesys::Peer;
use std::env;

fn main() -> Result<(), ()> {

	// Let's first check and see if we have a config file as a command line argument
	if env::args().count() >= 2 {
		// We have one, so load it and away we go.
		let conf_file = env::args().nth(1).unwrap();
		tesys::loggable::log(&format!("Loading configuration: {}", conf_file));
	} else {
		// We don't have one, so display an error.
		tesys::loggable::err("No configuration file specified. ");
		tesys::loggable::err("Please give the configuration file as the first argument. ");
	}

    tesys::loggable::log("Starting Tesys...");
    let mut _an: Angle = Angle::new(57.4874099038135);
    tesys::loggable::log(&format!("{}", _an));
    let _hms: HMS = _an.into();
    tesys::loggable::log(&format!("{}", _hms));
    let _an2: Angle = Angle::new_from_hms(_hms);

    tesys::loggable::log("Initialising Peer...");
    let _p = Peer::new();
	Ok(())
}
