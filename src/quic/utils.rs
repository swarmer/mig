/// Return the number with all except <byte_count> least-significant bytes set to zero
///
/// This is useful when truncating packet numbers
/// so that byteorder's write_uint doesn't panic
pub fn truncate_u64(number: u64, byte_count: usize) -> u64 {
    number % (1 << (byte_count * 8))
}