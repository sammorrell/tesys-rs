#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tesys_derive;

pub extern crate tesys_astrometry as astrometry;

mod peer;
pub use self::peer::Peer;

pub mod loggable;
pub use self::loggable::Loggable;

pub mod routable;
pub use self::routable::Routable;

#[macro_use]
pub mod plugin;
pub use self::plugin::Plugin;

pub mod pluginmanager;
pub use self::pluginmanager::PluginManager;
