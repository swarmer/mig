use std::io;

use byteorder::{BigEndian, WriteBytesExt};
use cast;

use quic::errors::Result;


pub const FRAME_GOAWAY: u8 = 0x03;

#[derive(Clone, Debug, Default)]
pub struct GoAwayFrame {
    pub error_code: u32,
    pub last_good_stream_id: u32,
    pub reason_phrase: Option<String>,
}

impl GoAwayFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        write.write_u8(FRAME_GOAWAY)?;

        write.write_u32::<BigEndian>(self.error_code)?;
        write.write_u32::<BigEndian>(self.last_good_stream_id)?;

        match self.reason_phrase {
            Some(ref reason_string) => {
                write.write_u16::<BigEndian>(
                    cast::u16(reason_string.len())
                    .expect("Reason phrase too long, length has to fit in 16 bits")
                )?;
                write.write_all(reason_string.as_bytes())?;
            },
            None => {
                write.write_u16::<BigEndian>(0)?;
            }
        }

        Ok(())
    }
}
