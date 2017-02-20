use std::io;

use byteorder::{BigEndian, WriteBytesExt};


pub const FRAME_BLOCKED: u8 = 0x05;

pub struct BlockedFrame {
    pub stream_id: u32,
}

impl BlockedFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        write.write_u8(FRAME_BLOCKED)?;

        write.write_u32::<BigEndian>(self.stream_id)?;

        Ok(())
    }
}
