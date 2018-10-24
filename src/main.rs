extern crate tesys;
extern crate chrono;

use std::env;
use tesys::astrometry::{SkyCoordinate, Location};
use tesys::Peer;
use chrono::Local;
use tesys::astrometry::datetime::*;

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

    let coord = SkyCoordinate::new(57.4874099038135, 57.4874099038135);
    tesys::loggable::log(&format!("{}", coord));
    let ang = coord.ra.clone();
    tesys::loggable::log(&format!("{}", ang));

    let dt = Local::now();
    let loc = Location::new(50.73778, -3.535278);
    println!("{}", datetime_to_modified_julian_date(dt));
    println!("{}", get_sidereal_time(dt, loc).to_hms());

    tesys::loggable::log("Initialising Peer...");
    let mut _p = Peer::new();
    _p.load_plugins();
    Ok(())
}
