use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::utils::map_unexpected_eof;


pub const FRAME_BLOCKED: u8 = 0x05;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BlockedFrame {
    pub stream_id: u32,
}

impl BlockedFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_BLOCKED)?;

        write.write_u32::<BigEndian>(self.stream_id)?;

        Ok(())
    }

    pub fn decode(read: &mut io::Read) -> Result<BlockedFrame> {
        if read.read_u8().map_err(map_unexpected_eof)? != FRAME_BLOCKED {
            panic!("Incorrect frame's decode called!")
        }

        let stream_id = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;

        Ok(BlockedFrame { stream_id: stream_id })
    }
}
