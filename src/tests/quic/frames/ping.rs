use std::io;

use quic::errors::Error;
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

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x07,
        ]
    );
    let frame = ping::PingFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        ping::PingFrame {}
    );

    let mut read = io::Cursor::new(
        vec![]
    );
    match ping::PingFrame::decode(&mut read) {
        Err(Error::Decoding(_)) => {},
        _ => assert!(false, "Error expected"),
    };
}
