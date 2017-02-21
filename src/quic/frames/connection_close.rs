use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::utils::map_unexpected_eof;
use super::utils::{encode_reason_phrase, decode_reason_phrase};


pub const FRAME_CONNECTION_CLOSE: u8 = 0x02;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectionCloseFrame {
    pub error_code: u32,
    pub reason_phrase: Option<String>,
}

impl ConnectionCloseFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_CONNECTION_CLOSE)?;

        write.write_u32::<BigEndian>(self.error_code)?;
        encode_reason_phrase(write, &self.reason_phrase)?;

        Ok(())
    }

    pub fn decode(read: &mut io::Read) -> Result<ConnectionCloseFrame> {
        if read.read_u8().map_err(map_unexpected_eof)? != FRAME_CONNECTION_CLOSE {
            panic!("Incorrect frame's decode called!")
        }

        let error_code = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let reason_phrase = decode_reason_phrase(read)?;

        Ok(ConnectionCloseFrame {
            error_code: error_code,
            reason_phrase: reason_phrase,
        })
    }
}
