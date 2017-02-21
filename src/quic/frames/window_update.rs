use std::io;

use byteorder::{BigEndian, WriteBytesExt};

use quic::errors::Result;


pub const FRAME_WINDOW_UPDATE: u8 = 0x04;

#[derive(Clone, Debug, Default)]
pub struct WindowUpdateFrame {
    pub stream_id: u32,
    pub byte_offset: u64,
}

impl WindowUpdateFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_WINDOW_UPDATE)?;

        write.write_u32::<BigEndian>(self.stream_id)?;
        write.write_u64::<BigEndian>(self.byte_offset)?;

        Ok(())
    }
}