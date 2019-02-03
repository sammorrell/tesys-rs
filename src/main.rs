extern crate chrono;
#[macro_use]
extern crate tesys;
pub use tesys::loggable;

use chrono::prelude::*;
use chrono::Local;
use std::env;
use tesys::astrometry::datetime::*;
use tesys::astrometry::frame::CanTransformTo;
use tesys::astrometry::frames::{FK5, ICRS};
use tesys::astrometry::{Epoch, Frame, Location, ProperMotion, SkyCoordinate};
use tesys::Peer;
use tesys::net::{Message,Route};

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

    let coord = SkyCoordinate::<ICRS>::new(279.23473479, 38.78368896)
        .with_epoch(Epoch::j2000())
        .with_proper_motion(ProperMotion::new(200.94, 286.23)); // Vega
    let c1 = coord.transform_to(FK5::new()).finish();
    tesys::loggable::log(&format!("{}", coord));
    tesys::loggable::log(&format!("{}", c1));
    let ang = coord.ra();
    tesys::loggable::log(&format!("{}", ang));

    let dt = Local::now();
    let loc = Location::new(50.73778, -3.535278);
    tesys::loggable::log(&format!(
        "{}",
        datetime_to_modified_julian_date(dt.with_timezone(&Utc))
    ));
    tesys::loggable::log(&format!(
        "{}",
        get_sidereal_time(dt.with_timezone(&Utc), loc.clone()).to_hms()
    ));
    tesys::loggable::log(&format!("{}", coord.to_sky_position(dt, loc.clone())));

    // Testing message
    let m = Message::new().to(Route::from_str("test-peer.test")).with_payload(coord.clone()).finish();
    let dat: SkyCoordinate<ICRS> = match m.get_payload() {
        Ok(p) => p,
        Err(_) => SkyCoordinate::<ICRS>::new(0., 0.),
    };
    tesys_warn!("{:?}", m);
    tesys_warn!("{:?}", dat);


    tesys::loggable::log("Initialising Peer...");
    let mut _p = Peer::new();
    _p.load_plugins();
    _p.run();

    Ok(())
}
