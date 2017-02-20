use std::io;

use quic::frames::window_update;


#[test]
fn test_encoding() {
    let frame = window_update::WindowUpdateFrame {
        stream_id: 42,
        byte_offset: 32,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
        ]
    );
}
