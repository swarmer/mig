use std::io;

use quic::QUIC_VERSION;
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
pub struct PacketPayload {
    // TODO: add TLS stuff one day
    pub frames: Vec<Frame>,
}

impl PacketPayload {
    pub fn encode<W: io::Write>(&self, write: &mut W, packet_number_size: usize) -> Result<()> {
        assert!(!self.frames.is_empty());

        for frame in &self.frames {
            // TODO: pass correct last_frame
            frame.encode(write, packet_number_size, false)?;
        }

        Ok(())
    }

    pub fn decode<R: io::Read + io::Seek>(read: &mut R, packet_number_size: usize) -> Result<PacketPayload> {
        let mut frames = Vec::new();
        loop {
            match Frame::decode(read, packet_number_size) {
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

        Ok(PacketPayload { frames: frames })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum PacketBody {
    PublicReset,
    Regular {
        version: Option<u32>,
        packet_number: u64,
        payload: PacketPayload,
    },
    VersionNegotiation {
        versions: Vec<u32>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    // flags
    pub has_version: bool,
    pub public_reset: bool,
    pub key_phase: bool,
    pub has_connection_id: bool,
    pub packet_number_size: usize,
    pub multipath: bool,

    pub connection_id: Option<u64>,
    pub packet_body: PacketBody,
}
