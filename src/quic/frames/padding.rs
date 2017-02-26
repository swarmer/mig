use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use quic::errors::Result;


pub const FRAME_PADDING: u8 = 0x00;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PaddingFrame {}

impl PaddingFrame {
    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        write.write_u8(FRAME_PADDING)?;

        Ok(())
    }

    pub fn decode<R: io::Read>(read: &mut R) -> Result<PaddingFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_PADDING);

        Ok(PaddingFrame {})
    }
}
