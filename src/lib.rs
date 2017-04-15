// Dependencies
extern crate byteorder;
extern crate cast;
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;
extern crate rustc_serialize;


// Submodules
pub mod benchmarks;
pub mod quic;


// Version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
