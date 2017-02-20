use std::io;

use quic::frames::rst_stream;


#[test]
fn test_encoding() {
    let frame = rst_stream::RstStreamFrame {
        error_code: 42,
        stream_id: 32,
        final_offset: 64,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x01,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        ]
    );
}
