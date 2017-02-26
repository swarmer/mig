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
    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        write.write_u8(FRAME_BLOCKED)?;

        write.write_u32::<BigEndian>(self.stream_id)?;

        Ok(())
    }

    pub fn decode<R: io::Read>(read: &mut R) -> Result<BlockedFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_BLOCKED);

        let stream_id = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;

        Ok(BlockedFrame { stream_id: stream_id })
    }
}
