use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::utils::map_unexpected_eof;


pub const FRAME_WINDOW_UPDATE: u8 = 0x04;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowUpdateFrame {
    pub stream_id: u32,
    pub byte_offset: u64,
}

impl WindowUpdateFrame {
    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        write.write_u8(FRAME_WINDOW_UPDATE)?;

        write.write_u32::<BigEndian>(self.stream_id)?;
        write.write_u64::<BigEndian>(self.byte_offset)?;

        Ok(())
    }

    pub fn decode<R: io::Read>(read: &mut R) -> Result<WindowUpdateFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_WINDOW_UPDATE);

        let stream_id = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let byte_offset = 
            read.read_u64::<BigEndian>()
            .map_err(map_unexpected_eof)?;

        Ok(WindowUpdateFrame {
            stream_id: stream_id,
            byte_offset: byte_offset,
        })
    }
}
