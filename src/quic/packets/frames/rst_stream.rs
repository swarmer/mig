use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::packets::utils::map_unexpected_eof;


pub const FRAME_RST_STREAM: u8 = 0x01;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RstStreamFrame {
    pub error_code: u32,
    pub stream_id: u32,
    pub final_offset: u64,
}

impl RstStreamFrame {
    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        write.write_u8(FRAME_RST_STREAM)?;

        write.write_u32::<BigEndian>(self.error_code)?;
        write.write_u32::<BigEndian>(self.stream_id)?;
        write.write_u64::<BigEndian>(self.final_offset)?;

        Ok(())
    }

    pub fn decode<R: io::Read>(read: &mut R) -> Result<RstStreamFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_RST_STREAM);

        let error_code = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let stream_id = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let final_offset = 
            read.read_u64::<BigEndian>()
            .map_err(map_unexpected_eof)?;

        Ok(RstStreamFrame {
            error_code: error_code,
            stream_id: stream_id,
            final_offset: final_offset,
        })
    }
}
