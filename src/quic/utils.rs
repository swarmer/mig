use std::io;

use quic::errors::Error;


/// Return the number with all except <byte_count> least-significant bytes set to zero
///
/// This is useful when truncating packet numbers
/// so that byteorder's write_uint doesn't panic
pub fn truncate_u64(number: u64, byte_count: usize) -> u64 {
    number % (1 << (byte_count * 8))
}

/// Wrap an UnexpectedEof io error into our own Decoding error
pub fn map_unexpected_eof(io_error: io::Error) -> Error {
    if let io::ErrorKind::UnexpectedEof = io_error.kind() {
        Error::Decoding("Unexpected packet end when decoding a frame".to_string())
    } else {
        Error::Io(io_error)
    }
}
