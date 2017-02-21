use std::io;

use byteorder::{WriteBytesExt};

use quic::errors::Result;


pub const FRAME_PING: u8 = 0x07;

#[derive(Clone, Copy, Debug, Default)]
pub struct PingFrame {}

impl PingFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_PING)?;

        Ok(())
    }
}
