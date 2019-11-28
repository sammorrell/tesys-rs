pub use tesys::loggable;

use std::env;
use tesys::Peer;

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

    // Initialise the peer
    tesys::loggable::log("Initialising Peer...");
    let mut _p = Peer::new();
    _p.load_plugins();
    _p.run()
}
