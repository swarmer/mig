use std::io;

use quic::errors::Error;
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

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
        ]
    );
    let frame = window_update::WindowUpdateFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        window_update::WindowUpdateFrame {
            stream_id: 42,
            byte_offset: 32,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    );
    match window_update::WindowUpdateFrame::decode(&mut read) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };
}
