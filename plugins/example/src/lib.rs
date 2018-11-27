#[macro_use]
extern crate tesys;
#[macro_use]
extern crate tesys_derive;
use tesys::Loggable;
use tesys::astrometry::SkyCoordinate;
use tesys::astrometry::frames::ICRS;
use tesys::loggable;
use tesys::Plugin;
use tesys::Routable;

// We call into the macros to write the extern C functions
// which allow us to easily create and destroy our Rust
// plugin struct. 
tesys_plugin_create!(ExamplePlugin);
tesys_plugin_destroy!(ExamplePlugin);

tesys_plugin!(ExamplePlugin {
    label: String,
    coord: SkyCoordinate<ICRS>,
});

impl Plugin for ExamplePlugin {
    /*
    fn new() -> ExamplePlugin {
        ExamplePlugin {
            label: "".to_string(),
            coord: SkyCoordinate::<ICRS>::new(0.0, 0.0),
        }
    }*/
    tesys_plugin_new!(
        label: "".to_string(),
        coord: SkyCoordinate::<ICRS>::new(0.0, 0.0),
    );

    fn test(&mut self) {
        tesys_log!(Self, "Test: {}", self.test_field);
        tesys_warn!(Self, "{}", self.coord);
        self.coord.coords[0] += 137.6;
        self.coord.coords[1] += 86.3;
        tesys_warn!(Self, "{}", self.coord);
        test_test(self);
    }
}

fn test_test( pg: &mut ExamplePlugin) {
    println!("Testing function call.");
    println!("{}", pg.test_field);
}