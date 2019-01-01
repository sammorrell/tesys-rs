#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tesys_derive;

#[macro_use]
pub mod loggable;
pub use crate::loggable::Loggable;

pub extern crate tesys_astrometry as astrometry;

mod peer;
pub use crate::peer::Peer;

#[macro_use]
pub mod plugin;
pub use crate::plugin::Plugin;

mod pluginhost;
pub use crate::pluginhost::PluginHost;

mod pluginmanager;
pub use crate::pluginmanager::PluginManager;

// Forwarding modules for the crate
pub mod net;
pub mod codegen;
pub mod timing;
