use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::utils::map_unexpected_eof;


pub const FRAME_PADDING: u8 = 0x00;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PaddingFrame {}

impl PaddingFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_PADDING)?;

        Ok(())
    }

    pub fn decode(read: &mut io::Read) -> Result<PaddingFrame> {
        if read.read_u8().map_err(map_unexpected_eof)? != FRAME_PADDING {
            panic!("Incorrect frame's decode called!")
        }

        Ok(PaddingFrame {})
    }
}
