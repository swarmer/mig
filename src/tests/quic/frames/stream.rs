use std::io;

use quic::frames::stream;


#[test]
fn test_encoding() {
    let frame = stream::StreamFrame {
        stream_id: 42,
        offset: 32,
        stream_data: vec![0x68, 0x65, 0x6C, 0x6C, 0x6F],
        fin: false,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write, false).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0xBF,
            0x00, 0x05,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
            0x68, 0x65, 0x6C, 0x6C, 0x6F,
        ]
    );

    let frame = stream::StreamFrame {
        stream_id: 0xDEADCAFE,
        offset: 0x10111213DEADCAFE,
        stream_data: vec![0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x32],
        fin: true,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write, true).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0xDF,
            0xDE, 0xAD, 0xCA, 0xFE,
            0x10, 0x11, 0x12, 0x13, 0xDE, 0xAD, 0xCA, 0xFE,
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x32,
        ]
    );
}
