use std::io;

use quic::frames::goaway;


#[test]
fn test_encoding() {
    let frame = goaway::GoAwayFrame {
        error_code: 42,
        last_good_stream_id: 32,
        reason_phrase: None,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x03,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00,
        ]
    );

    let frame = goaway::GoAwayFrame {
        error_code: 42,
        last_good_stream_id: 32,
        reason_phrase: Some("hello".to_string()),
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x03,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x05,
            0x68, 0x65, 0x6C, 0x6C, 0x6F,
        ]
    );
}
