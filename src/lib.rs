// Dependencies
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rustc_serialize;


// Submodules
#[cfg(test)]
mod tests;
pub mod demo;
pub mod quic;


// Version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
