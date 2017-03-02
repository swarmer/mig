mod frames;

use std::io;

use quic::QUIC_VERSION;
use quic::endpoint::EndpointRole;
use quic::errors::Error;
use quic::packets::frames::{Frame, padding, ping};
use quic::packets;


#[test]
fn test_payload_encoding() {
    let payload = packets::PacketPayload {
        frames: vec![
            Frame::Padding(padding::PaddingFrame {}),
            Frame::Ping(ping::PingFrame {}),
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
                Frame::Padding(padding::PaddingFrame {}),
                Frame::Ping(ping::PingFrame {}),
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


#[test]
fn test_packet_encoding() {
    let packet = packets::Packet::Regular(
        packets::RegularPacket {
            header: packets::PacketHeader {
                key_phase: true,
                packet_number_size: 4,
                multipath: true,

                connection_id: Some(0xABCDEF1234567890),
            },

            version: Some(0x12345678),
            packet_number: 0x1234567890ABCDEF,
            payload: packets::PacketPayload {
                frames: vec![
                    Frame::Padding(padding::PaddingFrame {}),
                    Frame::Ping(ping::PingFrame {}),
                ],
            },
        }
    );
    let mut write = io::Cursor::new(Vec::new());
    packet.encode(&mut write).unwrap();
    assert_eq!(
        write.into_inner(),
        vec![
            // header
            0x6D,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // regular packet fields
            0x12, 0x34, 0x56, 0x78,
            0x90, 0xAB, 0xCD, 0xEF,

            // payload
            0x00,
            0x07,
        ]
    );

    let packet = packets::Packet::Regular(
        packets::RegularPacket {
            header: packets::PacketHeader {
                key_phase: false,
                packet_number_size: 4,
                multipath: false,

                connection_id: None,
            },

            version: None,
            packet_number: 0x1234567890ABCDEF,
            payload: packets::PacketPayload {
                frames: vec![
                    Frame::Padding(padding::PaddingFrame {}),
                    Frame::Ping(ping::PingFrame {}),
                ],
            },
        }
    );
    let mut write = io::Cursor::new(Vec::new());
    packet.encode(&mut write).unwrap();
    assert_eq!(
        write.into_inner(),
        vec![
            // header
            0x20,

            // regular packet fields
            0x90, 0xAB, 0xCD, 0xEF,

            // payload
            0x00,
            0x07,
        ]
    );

    let packet = packets::Packet::VersionNegotiation(
        packets::VersionNegotiationPacket {
            header: packets::PacketHeader {
                key_phase: false,
                packet_number_size: 1,
                multipath: false,

                connection_id: Some(0xABCDEF1234567890),
            },

            versions: vec![0x12345678, 0xABCDEF12],
        }
    );
    let mut write = io::Cursor::new(Vec::new());
    packet.encode(&mut write).unwrap();
    assert_eq!(
        write.into_inner(),
        vec![
            // header
            0x09,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // versions
            0x12, 0x34, 0x56, 0x78,
            0xAB, 0xCD, 0xEF, 0x12,
        ]
    );

    let packet = packets::Packet::PublicReset(
        packets::PublicResetPacket {
            header: packets::PacketHeader {
                key_phase: false,
                packet_number_size: 1,
                multipath: false,

                connection_id: Some(0xABCDEF1234567890),
            },
        }
    );
    let mut write = io::Cursor::new(Vec::new());
    packet.encode(&mut write).unwrap();
    assert_eq!(
        write.into_inner(),
        vec![
            // header
            0x0A,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,
        ]
    );
}

#[test]
fn test_packet_decoding() {
    let mut read = io::Cursor::new(
        vec![
            // header
            0x6D,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // regular packet fields
            0xFA, 0xB0, 0x00, 0x01,
            0x90, 0xAB, 0xCD, 0xEF,

            // payload
            0x00,
            0x07,
        ]
    );
    assert_eq!(
        packets::Packet::decode(&mut read, EndpointRole::Server).unwrap(),
        packets::Packet::Regular(
            packets::RegularPacket {
                header: packets::PacketHeader {
                    key_phase: true,
                    packet_number_size: 4,
                    multipath: true,

                    connection_id: Some(0xABCDEF1234567890),
                },

                version: Some(QUIC_VERSION),
                packet_number: 0x0000000090ABCDEF,
                payload: packets::PacketPayload {
                    frames: vec![
                        Frame::Padding(padding::PaddingFrame {}),
                        Frame::Ping(ping::PingFrame {}),
                    ],
                },
            }
        )
    );

    let mut read = io::Cursor::new(
        vec![
            // header
            0x20,

            // regular packet fields
            0x90, 0xAB, 0xCD, 0xEF,

            // payload
            0x00,
            0x07,
        ]
    );
    assert_eq!(
        packets::Packet::decode(&mut read, EndpointRole::Server).unwrap(),
        packets::Packet::Regular(
            packets::RegularPacket {
                header: packets::PacketHeader {
                    key_phase: false,
                    packet_number_size: 4,
                    multipath: false,

                    connection_id: None,
                },

                version: None,
                packet_number: 0x0000000090ABCDEF,
                payload: packets::PacketPayload {
                    frames: vec![
                        Frame::Padding(padding::PaddingFrame {}),
                        Frame::Ping(ping::PingFrame {}),
                    ],
                },
            }
        )
    );

    let mut read = io::Cursor::new(
        vec![
            // header
            0x09,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // versions
            0x12, 0x34, 0x56, 0x78,
            0xAB, 0xCD, 0xEF, 0x12,
        ]
    );
    assert_eq!(
        packets::Packet::decode(&mut read, EndpointRole::Client).unwrap(),
        packets::Packet::VersionNegotiation(
            packets::VersionNegotiationPacket {
                header: packets::PacketHeader {
                    key_phase: false,
                    packet_number_size: 1,
                    multipath: false,

                    connection_id: Some(0xABCDEF1234567890),
                },

                versions: vec![0x12345678, 0xABCDEF12],
            }
        )
    );

    let mut read = io::Cursor::new(
        vec![
            // header
            0x0A,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,
        ]
    );
    assert_eq!(
        packets::Packet::decode(&mut read, EndpointRole::Server).unwrap(),
        packets::Packet::PublicReset(
            packets::PublicResetPacket {
                header: packets::PacketHeader {
                    key_phase: false,
                    packet_number_size: 1,
                    multipath: false,

                    connection_id: Some(0xABCDEF1234567890),
                },
            }
        )
    );

    let mut read = io::Cursor::new(
        vec![
            // header
            0x6D,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // regular packet fields
            0xFA, 0xC0, 0x00, 0x01,
            0x90, 0xAB, 0xCD, 0xEF,

            // payload
            0x00,
            0x07,
        ]
    );
    match packets::Packet::decode(&mut read, EndpointRole::Server) {
        Err(Error::UnsupportedVersion(..)) => {},
        _ => assert!(false, "UnsupportedVersion error expected"),
    };

    let mut read = io::Cursor::new(
        vec![
            // header
            0x09,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // versions
            0x12, 0x34, 0x56, 0x78,
            0xAB, 0xCD, 0xEF, 0x12,
        ]
    );
    match packets::Packet::decode(&mut read, EndpointRole::Server) {
        Err(Error::UnsupportedVersion(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };

    let mut read = io::Cursor::new(
        vec![
            // header
            0x6D,
            0xAB, 0xCD, 0xEF, 0x12, 0x34, 0x56, 0x78, 0x90,

            // regular packet fields
            0x12, 0x34, 0x56, 0x78,
        ]
    );
    match packets::Packet::decode(&mut read, EndpointRole::Server) {
        Err(Error::UnsupportedVersion(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };

    let mut read = io::Cursor::new(
        vec![]
    );
    match packets::Packet::decode(&mut read, EndpointRole::Server) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Decoding error expected"),
    };
}
