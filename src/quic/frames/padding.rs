use std::io;

use byteorder::{WriteBytesExt};

use quic::errors::Result;


pub const FRAME_PADDING: u8 = 0x00;

#[derive(Clone, Copy, Debug, Default)]
pub struct PaddingFrame {}

impl PaddingFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_PADDING)?;

        Ok(())
    }
}
