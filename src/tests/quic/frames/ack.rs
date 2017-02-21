use std::io;

use quic::errors::Error;
use quic::frames::ack;


#[test]
fn test_encoding() {
    let frame = ack::AckFrame {
        largest_acknowledged: 42,
        ack_delay: 32,

        first_ack_block_length: 64,
        extra_ack_blocks: Vec::new(),

        first_timestamp: None,
        extra_timestamps: Vec::new(),
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x4F,
            0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        ]
    );

    let frame = ack::AckFrame {
        largest_acknowledged: 42,
        ack_delay: 32,

        first_ack_block_length: 64,
        extra_ack_blocks: vec![
            ack::ExtraAckBlock { gap: 31, block_length: 0xDEADCAFE },
            ack::ExtraAckBlock { gap: 33, block_length: 0xAABBCCDDEEFF },
        ],

        first_timestamp: Some(ack::FirstAckTimestamp { delta_la: 1, delta_timestamp: 0xAABBCCDD }),
        extra_timestamps: vec![
            ack::ExtraAckTimestamp { delta_la: 2, delta_timestamp: 0xBBCC },
        ],
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.into_inner(),
        vec![
            0x6F,
            0x02,
            0x02,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
            0x1F,
            0x00, 0x00, 0xDE, 0xAD, 0xCA, 0xFE,
            0x21,
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,

            0x01,
            0xAA, 0xBB, 0xCC, 0xDD,
            0x02,
            0xBB, 0xCC,
        ]
    );
}

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x4F,
            0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        ]
    );
    let frame = ack::AckFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        ack::AckFrame {
            largest_acknowledged: 42,
            ack_delay: 32,

            first_ack_block_length: 64,
            extra_ack_blocks: Vec::new(),

            first_timestamp: None,
            extra_timestamps: Vec::new(),
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x6F,
            0x02,
            0x02,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
            0x1F,
            0x00, 0x00, 0xDE, 0xAD, 0xCA, 0xFE,
            0x21,
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,

            0x01,
            0xAA, 0xBB, 0xCC, 0xDD,
            0x02,
            0xBB, 0xCC,
        ]
    );
    let frame = ack::AckFrame::decode(&mut read).unwrap();
    assert_eq!(
        frame,
        ack::AckFrame {
            largest_acknowledged: 42,
            ack_delay: 32,

            first_ack_block_length: 64,
            extra_ack_blocks: vec![
                ack::ExtraAckBlock { gap: 31, block_length: 0xDEADCAFE },
                ack::ExtraAckBlock { gap: 33, block_length: 0xAABBCCDDEEFF },
            ],

            first_timestamp: Some(ack::FirstAckTimestamp { delta_la: 1, delta_timestamp: 0xAABBCCDD }),
            extra_timestamps: vec![
                ack::ExtraAckTimestamp { delta_la: 2, delta_timestamp: 0xBBCC },
            ],
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x6F,
            0x02,
            0x02,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
            0x1F,
            0x00, 0x00, 0xDE, 0xAD, 0xCA, 0xFE,
            0x21,
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,

            0x01,
            0xAA, 0xBB, 0xCC, 0xDD,
            0x02,
            0xBB,
        ]
    );
    match ack::AckFrame::decode(&mut read) {
        Err(Error::Decoding(_)) => {},
        _ => assert!(false, "Error expected"),
    };
}
