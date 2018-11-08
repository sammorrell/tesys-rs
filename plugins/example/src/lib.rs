extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::astrometry::SkyCoordinate;
use tesys::astrometry::frames::ICRS;
use tesys::loggable::*;
use tesys::Plugin;

#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut Plugin {
    let obj = ExamplePlugin::new();
    let boxed: Box<ExamplePlugin> = Box::new(obj);
    Box::into_raw(boxed)
}

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
