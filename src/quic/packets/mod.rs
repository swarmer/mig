pub mod frames;
mod utils;

use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::QUIC_VERSION;
use quic::errors::{Error, Result};
use quic::endpoint_role::EndpointRole;
use self::utils::{map_unexpected_eof, truncate_u64};
use self::frames::Frame;


pub const FLAG_VERSION: u8 = 0b00000001;
pub const FLAG_PUBLIC_RESET: u8 = 0b00000010;
pub const FLAG_KEY_PHASE: u8 = 0b00000100;
pub const FLAG_CONNECTION_ID: u8 = 0b00001000;
pub const FLAG_MULTIPATH: u8 = 0b01000000;

pub const MASK_PACKET_NUMBER_SIZE: u8 = 0b00110000;


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


#[derive(Clone, Debug, Default, PartialEq)]
pub struct PacketHeader {
    // flags
    pub key_phase: bool,
    pub packet_number_size: usize,
    pub multipath: bool,

    pub connection_id: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PublicResetPacket {
    pub header: PacketHeader,

    // TBD
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegularPacket {
    pub header: PacketHeader,

    pub version: Option<u32>,
    pub packet_number: u64,
    pub payload: PacketPayload,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VersionNegotiationPacket {
    pub header: PacketHeader,

    pub versions: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Packet {
    Regular(RegularPacket),
    VersionNegotiation(VersionNegotiationPacket),
    PublicReset(PublicResetPacket),
}

impl Packet {
    fn encode_header<W>(write: &mut W, header: &PacketHeader, have_version: bool, public_reset: bool) -> Result<()>
            where W: io::Write {
        // flags word
        let mut flags = 0b00000000;
        if have_version {
            flags |= FLAG_VERSION;
        }
        if public_reset {
            flags |= FLAG_PUBLIC_RESET;
        }
        if header.key_phase {
            flags |= FLAG_KEY_PHASE;
        }
        if header.connection_id.is_some() {
            flags |= FLAG_CONNECTION_ID;
        }
        flags |= match header.packet_number_size {
            1 => 0b00 << 4,
            2 => 0b01 << 4,
            4 => 0b10 << 4,
            6 => 0b11 << 4,
            _ => panic!("Invalid packet number size"),
        };
        if header.multipath {
            flags |= FLAG_MULTIPATH;
        }
        write.write_u8(flags)?;

        // connection id
        if let Some(connection_id) = header.connection_id {
            write.write_u64::<BigEndian>(connection_id)?;
        }

        Ok(())
    }

    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        match *self {
            Packet::Regular(ref regular_packet) => {
                Packet::encode_header(
                    write,
                    &regular_packet.header,
                    regular_packet.version.is_some(),
                    false,
                )?;

                if let Some(version) = regular_packet.version {
                    write.write_u32::<BigEndian>(version)?;
                }

                let packet_number_size = regular_packet.header.packet_number_size;
                write.write_uint::<BigEndian>(
                    truncate_u64(regular_packet.packet_number, packet_number_size),
                    packet_number_size,
                )?;

                regular_packet.payload.encode(write, packet_number_size)?;
            },
            Packet::VersionNegotiation(ref version_packet) => {
                Packet::encode_header(
                    write,
                    &version_packet.header,
                    true,
                    false,
                )?;

                for &version in &version_packet.versions {
                    write.write_u32::<BigEndian>(version)?;
                }
            },
            Packet::PublicReset(ref public_reset_packet) => {
                Packet::encode_header(
                    write,
                    &public_reset_packet.header,
                    false,
                    true,
                )?;
            },
        };

        Ok(())
    }

    pub fn decode<R: io::Read + io::Seek>(read: &mut R, endpoint_type: EndpointRole) -> Result<Packet> {
        let flags = read.read_u8().map_err(map_unexpected_eof)?;
        let has_version = (flags & FLAG_VERSION) != 0;
        let public_reset = (flags & FLAG_PUBLIC_RESET) != 0;
        let key_phase = (flags & FLAG_KEY_PHASE) != 0;
        let has_connection_id = (flags & FLAG_CONNECTION_ID) != 0;
        let packet_number_size = match (flags & MASK_PACKET_NUMBER_SIZE) >> 4 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            0b11 => 6,
            _ => unreachable!(),
        };
        let multipath = (flags & FLAG_MULTIPATH) != 0;

        let connection_id = if has_connection_id {
            Some(read.read_u64::<BigEndian>().map_err(map_unexpected_eof)?)
        } else {
            None
        };

        let header = PacketHeader {
            key_phase: key_phase,
            packet_number_size: packet_number_size,
            multipath: multipath,

            connection_id: connection_id,
        };

        match (public_reset, has_version, endpoint_type) {
            (true, _, _) => {
                // public reset packet
                Ok(Packet::PublicReset(PublicResetPacket { header: header }))
            },
            (false, false, _) | (false, true, EndpointRole::Server) => {
                // regular packet
                let version = if has_version {
                    let version = read.read_u32::<BigEndian>().map_err(map_unexpected_eof)?;
                    if version != QUIC_VERSION {
                        return Err(Error::UnsupportedVersion(version));
                    }

                    Some(version)
                } else {
                    None
                };

                let packet_number =
                    read.read_uint::<BigEndian>(packet_number_size)
                    .map_err(map_unexpected_eof)?
                    as u64;
                
                let payload = PacketPayload::decode(read, packet_number_size)?;

                Ok(
                    Packet::Regular(
                        RegularPacket {
                            header: header,

                            version: version,
                            packet_number: packet_number,
                            payload: payload,
                        }
                    )
                )
            },
            (false, true, EndpointRole::Client) => {
                // version negotiation packet
                let mut versions = Vec::new();

                loop {
                    match read.read_u32::<BigEndian>() {
                        Ok(version) => {
                            versions.push(version);
                        },
                        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            break;
                        },
                        Err(e) => {
                            return Err(Error::from(e));
                        }
                    };
                }

                Ok(
                    Packet::VersionNegotiation(
                        VersionNegotiationPacket { header: header, versions: versions }
                    )
                )
            },
        }
    }
}
