use std::io;

use quic::errors::Error;
use quic::frames;
use quic::packets;


#[test]
fn test_payload_encoding() {
    let payload = packets::PacketPayload {
        frames: vec![
            frames::Frame::Padding(frames::padding::PaddingFrame {}),
            frames::Frame::Ping(frames::ping::PingFrame {}),
        ],
    };
    let mut write = io::Cursor::new(Vec::new());
    payload.encode(&mut write, 6).unwrap();
    assert_eq!(
        write.into_inner(),
        vec![
            // padding frame
            0x00,

            // ping frame
            0x07,
        ]
    );
}

#[test]
fn test_payload_decoding() {
    let mut read = io::Cursor::new(
        vec![
            // padding frame
            0x00,

            // ping frame
            0x07,
        ]
    );
    assert_eq!(
        packets::PacketPayload::decode(&mut read, 6).unwrap(),
        packets::PacketPayload {
            frames: vec![
                frames::Frame::Padding(frames::padding::PaddingFrame {}),
                frames::Frame::Ping(frames::ping::PingFrame {}),
            ]
        }
    );

    let mut read = io::Cursor::new(
        vec![
            // padding frame
            0x00,

            // incomplete window update frame
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    );
    match packets::PacketPayload::decode(&mut read, 6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };

    let mut read = io::Cursor::new(
        vec![]
    );
    match packets::PacketPayload::decode(&mut read, 6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };
}
