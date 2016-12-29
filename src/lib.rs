// Dependencies
#[macro_use]
extern crate log;
extern crate env_logger;


// Submodules
#[cfg(test)]
mod tests;
pub mod demo;


// Version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
