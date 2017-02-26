use std::io;

use quic::errors::Error;
use quic::frames::padding;


#[test]
fn test_encoding() {
    let frame = padding::PaddingFrame {};
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x00,
        ]
    );
}

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x00,
        ]
    );
    let frame = padding::PaddingFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        padding::PaddingFrame {}
    );

    let mut read = io::Cursor::new(
        vec![]
    );
    match padding::PaddingFrame::decode(&mut read) {
        Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => {},
        _ => assert!(false, "UnexpectedEof expected"),
    };
}
