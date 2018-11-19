#[macro_use]
extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::astrometry::SkyCoordinate;
use tesys::astrometry::frames::ICRS;
use tesys::loggable::*;
use tesys::Plugin;

// We call into the macros to write the extern C functions
// which allow us to easily create and destroy our Rust
// plugin struct. 
tesys_plugin_create!(ExamplePlugin);
tesys_plugin_destroy!(ExamplePlugin);

#[allow(dead_code)]
#[derive(Loggable)]
pub struct ExamplePlugin {
    label: String,
    coord: SkyCoordinate<ICRS>,
}

impl ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn new() -> ExamplePlugin {
        ExamplePlugin::log("Loading Plugin");
        ExamplePlugin {
            label: "".to_string(),
            coord: SkyCoordinate::<ICRS>::new(0.0, 0.0),
        }
    }

    fn test(&mut self) {
        Self::warn(&format!("{}", self.coord));
        self.coord.coords[0] += 137.6;
        self.coord.coords[1] += 86.3;
        Self::warn(&format!("{}", self.coord));
    }
}
