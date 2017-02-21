use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cast;

use quic::errors::Result;
use quic::utils::map_unexpected_eof;


pub const FRAME_FLAG_STREAM: u8 = 0b10000000;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StreamFrame {
    pub stream_id: u32,
    pub offset: u64,
    pub stream_data: Vec<u8>,
    pub fin: bool,
}

impl StreamFrame {
    pub fn encode(&self, write: &mut io::Write, last_frame: bool) -> Result<()> {
        // construct the type octet
        let mut frame_type = FRAME_FLAG_STREAM;

        if self.fin {
            frame_type |= 0b01000000
        }

        if !last_frame {
            frame_type |= 0b00100000;
        }

        // TODO: calculate this more intelligently
        let offset_size = 8;
        frame_type |= 0b00011100;

        // TODO: calculate this more intelligently
        let stream_id_size = 4;
        frame_type |= 0b00000011;

        write.write_u8(frame_type)?;

        // other fields
        if !last_frame {
            write.write_u16::<BigEndian>(
                cast::u16(self.stream_data.len())
                .expect("Stream data too big, size has to fit in 16 bits")
            )?;
        }
        write.write_uint::<BigEndian>(self.stream_id as u64, stream_id_size)?;
        write.write_uint::<BigEndian>(self.offset, offset_size)?;
        write.write_all(&self.stream_data[..])?;

        Ok(())
    }

    pub fn decode(read: &mut io::Read) -> Result<StreamFrame> {
        // extract type octet data
        let frame_type = read.read_u8().map_err(map_unexpected_eof)?;
        if (frame_type & FRAME_FLAG_STREAM) == 0 {
            panic!("Incorrect frame's decode called!")
        }

        let fin = (frame_type & 0b01000000) != 0;

        let last_frame = (frame_type & 0b00100000) == 0;

        let offset_size = match (frame_type & 0b00011100) >> 2 {
            0b000 => 0,
            bit_value => bit_value + 1,
        } as usize;

        let stream_id_size = ((frame_type & 0b00000011) + 1) as usize;

        // other fields
        let data_length = if !last_frame {
            Some(read.read_u16::<BigEndian>().map_err(map_unexpected_eof)? as usize)
        } else {
            None
        };

        let stream_id = 
            read.read_uint::<BigEndian>(stream_id_size)
            .map_err(map_unexpected_eof)?
            as u32;
        
        let offset = if offset_size != 0 {
            read.read_uint::<BigEndian>(offset_size)
            .map_err(map_unexpected_eof)?
        } else {
            0
        };

        let stream_data = match data_length {
            Some(data_length) => {
                let mut buffer = vec![0; data_length];
                read.read_exact(&mut buffer).map_err(map_unexpected_eof)?;
                buffer
            },
            None => {
                let mut buffer = Vec::new();
                read.read_to_end(&mut buffer).map_err(map_unexpected_eof)?;
                buffer
            },
        };

        Ok(StreamFrame {
            stream_id: stream_id,
            offset: offset,
            stream_data: stream_data,
            fin: fin,
        })
    }
}
