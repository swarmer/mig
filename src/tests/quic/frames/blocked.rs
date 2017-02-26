use std::io;

use quic::errors::Error;
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

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x05,
            0x00, 0x00, 0x00, 0x2A,
        ]
    );
    let frame = blocked::BlockedFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        blocked::BlockedFrame { stream_id: 42 }
    );

    let mut read = io::Cursor::new(
        vec![
            0x05,
            0x00, 0x00, 0x00,
        ]
    );
    match blocked::BlockedFrame::decode(&mut read) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };
}
