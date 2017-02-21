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
pub struct Payload {
    pub frames: Vec<Frame>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PacketBody {
    PublicReset,
    Regular {
        version: u32,
        packet_number: u64,
        payload: Payload,
    },
    VersionNegotiation {
        versions: Vec<u32>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    pub flags: u8,
    pub connection_id: Option<u64>,
    pub packet_body: PacketBody,
}
