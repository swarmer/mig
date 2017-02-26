use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use quic::errors::Result;


pub const FRAME_PING: u8 = 0x07;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PingFrame {}

impl PingFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_PING)?;

        Ok(())
    }

    pub fn decode(read: &mut io::Read) -> Result<PingFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_PING);

        Ok(PingFrame {})
    }
}
