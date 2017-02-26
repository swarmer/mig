use std::io;

use quic::errors::Error;
use quic::frames::connection_close;


#[test]
fn test_encoding() {
    let frame = connection_close::ConnectionCloseFrame {
        error_code: 42,
        reason_phrase: None,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00,
        ]
    );

    let frame = connection_close::ConnectionCloseFrame {
        error_code: 42,
        reason_phrase: Some("hello".to_string()),
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x05,
            0x68, 0x65, 0x6C, 0x6C, 0x6F,
        ]
    );
}

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00,
        ]
    );
    let frame = connection_close::ConnectionCloseFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        connection_close::ConnectionCloseFrame {
            error_code: 42,
            reason_phrase: None,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x05,
            0x68, 0x65, 0x6C, 0x6C, 0x6F,
        ]
    );
    let frame = connection_close::ConnectionCloseFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        connection_close::ConnectionCloseFrame {
            error_code: 42,
            reason_phrase: Some("hello".to_string()),
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x05,
            0x68, 0x65, 0x6C, 0x6C,
        ]
    );
    match connection_close::ConnectionCloseFrame::decode(&mut read) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };
}
