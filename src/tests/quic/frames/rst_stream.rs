use std::io;

use quic::errors::Error;
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

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x01,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        ]
    );
    let frame = rst_stream::RstStreamFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        rst_stream::RstStreamFrame {
            error_code: 42,
            stream_id: 32,
            final_offset: 64,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x01,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    );
    match rst_stream::RstStreamFrame::decode(&mut read) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };
}
