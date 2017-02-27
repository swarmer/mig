pub mod error_codes;
pub mod errors;
pub mod endpoint;
pub mod frames;
pub mod packets;

mod utils;


// temporary implementation-specific version
pub const QUIC_VERSION: u32 = 0xFAB00001;
