use quic::errors::Error;
use quic::frames;
use quic::packets;


#[test]
fn test_payload_encoding() {
    let decoded_payload = packets::DecodedPayload {
        frames: vec![
            frames::Frame::Padding(frames::padding::PaddingFrame {}),
            frames::Frame::Ping(frames::ping::PingFrame {}),
        ],
    };
    assert_eq!(
        decoded_payload.to_encoded(6),
        packets::EncodedPayload {
            bytes: vec![
                // padding frame
                0x00,

                // ping frame
                0x07,
            ]
        }
    );
}

#[test]
fn test_payload_decoding() {
    let encoded_payload = packets::EncodedPayload {
        bytes: vec![
            // padding frame
            0x00,

            // ping frame
            0x07,
        ],
    };
    assert_eq!(
        encoded_payload.to_decoded(6).unwrap(),
        packets::DecodedPayload {
            frames: vec![
                frames::Frame::Padding(frames::padding::PaddingFrame {}),
                frames::Frame::Ping(frames::ping::PingFrame {}),
            ]
        }
    );

    let encoded_payload = packets::EncodedPayload {
        bytes: vec![
            // padding frame
            0x00,

            // incomplete window update frame
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    };
    match encoded_payload.to_decoded(6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };

    let encoded_payload = packets::EncodedPayload {
        bytes: vec![],
    };
    match encoded_payload.to_decoded(6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };
}
