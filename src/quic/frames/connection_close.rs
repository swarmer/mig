use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cast;

use quic::errors::Result;
use quic::utils::map_unexpected_eof;


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

    pub fn decode(read: &mut io::Read) -> Result<ConnectionCloseFrame> {
        if read.read_u8().map_err(map_unexpected_eof)? != FRAME_CONNECTION_CLOSE {
            panic!("Incorrect frame's decode called!")
        }

        let error_code = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let reason_phrase_length = 
            read.read_u16::<BigEndian>()
            .map_err(map_unexpected_eof)?
            as usize;
        
        let reason_phrase = if reason_phrase_length != 0 {
            let mut buf = vec![0; reason_phrase_length];
            read.read_exact(&mut buf).map_err(map_unexpected_eof)?;
            let reason_phrase = String::from_utf8_lossy(&buf).to_string();
            Some(reason_phrase)
        } else {
            None
        };

        Ok(ConnectionCloseFrame {
            error_code: error_code,
            reason_phrase: reason_phrase,
        })
    }
}
