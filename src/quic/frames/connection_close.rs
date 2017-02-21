use std::io;

use byteorder::{BigEndian, WriteBytesExt};
use cast;


pub const FRAME_CONNECTION_CLOSE: u8 = 0x02;

#[derive(Clone, Debug, Default)]
pub struct ConnectionCloseFrame {
    pub error_code: u32,
    pub reason_phrase: Option<String>,
}

impl ConnectionCloseFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        write.write_u8(FRAME_CONNECTION_CLOSE)?;

        write.write_u32::<BigEndian>(self.error_code)?;

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
