use std::io;

use quic::frames::blocked;


#[test]
fn test_encoding() {
    let frame = blocked::BlockedFrame { stream_id: 42 };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x05,
            0x00, 0x00, 0x00, 0x2A,
        ]
    );
}
