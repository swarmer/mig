use std::io;

use byteorder::{WriteBytesExt};


pub const FRAME_PING: u8 = 0x07;

pub struct PingFrame {}

impl PingFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        write.write_u8(FRAME_PING)?;

        Ok(())
    }
}
