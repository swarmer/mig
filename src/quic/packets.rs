use std::io;

use quic::errors::{Error, Result};
use super::frames::Frame;


pub const FLAG_VERSION: u8 = 0b00000001;
pub const FLAG_PUBLIC_RESET: u8 = 0b00000010;
pub const FLAG_KEY_PHASE: u8 = 0b00000100;
pub const FLAG_CONNECTION_ID: u8 = 0b00001000;
pub const FLAG_PACKET_NUMBER_SIZE_1: u8 = 0b00010000;
pub const FLAG_PACKET_NUMBER_SIZE_2: u8 = 0b00100000;
pub const FLAG_MULTIPATH: u8 = 0b01000000;
pub const FLAG_UNUSED: u8 = 0b10000000;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct DecodedPayload {
    // TODO: add TLS stuff one day
    pub frames: Vec<Frame>,
}

impl DecodedPayload {
    pub fn to_encoded(&self, packet_number_size: usize) -> EncodedPayload {
        assert!(!self.frames.is_empty());

        let mut write = io::Cursor::new(Vec::new());
        for frame in &self.frames {
            // TODO: pass correct last_frame
            frame.encode(&mut write, packet_number_size, false).unwrap();
        }

        EncodedPayload { bytes: write.into_inner() }
    }
}


#[derive(Clone, Debug, Default, PartialEq)]
pub struct EncodedPayload {
    pub bytes: Vec<u8>,
}

impl EncodedPayload {
    pub fn to_decoded(&self, packet_number_size: usize) -> Result<DecodedPayload> {
        let mut read = io::Cursor::new(&self.bytes);
        let mut frames = Vec::new();
        loop {
            match Frame::decode(&mut read, packet_number_size) {
                Ok(frame) => {
                    frames.push(frame);
                },
                Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    break;
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if frames.is_empty() {
            return Err(Error::Decoding(String::from("At least one frame expected")));
        }

        Ok(DecodedPayload { frames: frames })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum PacketBody<P> {
    PublicReset,
    Regular {
        version: u32,
        packet_number: u64,
        payload: P,
    },
    VersionNegotiation {
        versions: Vec<u32>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Packet<P> {
    // flags
    pub has_version: bool,
    pub public_reset: bool,
    pub key_phase: bool,
    pub has_connection_id: bool,
    pub packet_number_size: usize,
    pub multipath: bool,

    pub connection_id: Option<u64>,
    pub packet_body: PacketBody<P>,
}
