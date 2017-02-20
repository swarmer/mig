use std::io;

use byteorder::{BigEndian, WriteBytesExt};


pub const FRAME_RST_STREAM: u8 = 0x01;

#[derive(Clone, Copy, Debug)]
pub struct RstStreamFrame {
    pub error_code: u32,
    pub stream_id: u32,
    pub final_offset: u64,
}

impl RstStreamFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        write.write_u8(FRAME_RST_STREAM)?;

        write.write_u32::<BigEndian>(self.error_code)?;
        write.write_u32::<BigEndian>(self.stream_id)?;
        write.write_u64::<BigEndian>(self.final_offset)?;

        Ok(())
    }
}
