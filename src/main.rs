#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tesys_derive;

mod tesys;
use tesys::astrometry::Angle;
use tesys::Peer;
use std::env;

fn main() -> Result<(), std::io::Error> {

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
    let mut _an: Angle = Angle::new(12.0);
    let _hms = _an.to_hms();
    println!("{}", _hms);
    let _an2: Angle = Angle::new_from_hms(_hms);

    tesys::loggable::log("Initialising Peer...");
    let _p = Peer::new();
	Ok(())
}
