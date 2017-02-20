use std::io;

use quic::frames::ping;


#[test]
fn test_encoding() {
    let frame = ping::PingFrame {};
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x07,
        ]
    );
}
