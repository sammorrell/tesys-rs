#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;
pub mod astrom;

pub mod astrometry;

// Use the module for unit tests if we want it. 
#[cfg(test)]
mod tests;