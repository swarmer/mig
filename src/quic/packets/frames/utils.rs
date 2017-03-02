use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cast;

use quic::errors::Result;
use quic::packets::utils::map_unexpected_eof;


pub fn encode_reason_phrase(write: &mut io::Write, reason_phrase: &Option<String>) -> Result<()> {
    match *reason_phrase {
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

pub fn decode_reason_phrase(read: &mut io::Read) -> Result<Option<String>> {
    let reason_phrase_length = 
        read.read_u16::<BigEndian>()
        .map_err(map_unexpected_eof)?
        as usize;
    
    if reason_phrase_length != 0 {
        let mut buf = vec![0; reason_phrase_length];
        read.read_exact(&mut buf).map_err(map_unexpected_eof)?;
        let reason_phrase = String::from_utf8_lossy(&buf).to_string();
        Ok(Some(reason_phrase))
    } else {
        Ok(None)
    }
}


pub fn check_packet_number_size(packet_number_size: usize) {
    match packet_number_size {
        1 | 2 | 4 | 6 => {},
        _ => panic!("Invalid packet number size: {}", packet_number_size),
    }
}
