use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use super::utils::check_packet_number_size;
use quic::packets::utils::map_unexpected_eof;


pub const FRAME_STOP_WAITING: u8 = 0x06;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StopWaitingFrame {
    pub least_acked_delta: u64,
}

impl StopWaitingFrame {
    pub fn encode<W: io::Write>(&self, write: &mut W, packet_number_size: usize) -> Result<()> {
        write.write_u8(FRAME_STOP_WAITING)?;

        check_packet_number_size(packet_number_size);
        write.write_uint::<BigEndian>(self.least_acked_delta, packet_number_size)?;

        Ok(())
    }

    pub fn decode<R: io::Read>(read: &mut R, packet_number_size: usize) -> Result<StopWaitingFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_STOP_WAITING);

        let least_acked_delta = 
            read.read_uint::<BigEndian>(packet_number_size)
            .map_err(map_unexpected_eof)?;

        Ok(StopWaitingFrame { least_acked_delta: least_acked_delta })
    }
}
