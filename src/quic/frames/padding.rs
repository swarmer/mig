use std::io;

use byteorder::{WriteBytesExt};


pub const FRAME_PADDING: u8 = 0x00;

pub struct PaddingFrame {}

impl PaddingFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        write.write_u8(FRAME_PADDING)?;

        Ok(())
    }
}
