use std::io;

use byteorder::{BigEndian, WriteBytesExt};
use cast;


pub const FRAME_FLAG_ACK: u8 = 0b01000000;

pub struct AckBlock {
    pub gap: Option<u8>,
    pub block_length: u64,
}

pub enum AckTimestamp {
    Delta(u8),
    FirstTimeStamp(u32),
    TimeSincePrevious(u16),
}

pub struct AckFrame {
    num_timestamps: u8,
    largest_acknowledged: u64,
    ack_delay: u16,
    ack_blocks: Vec<AckBlock>,
    timestamps: Vec<AckTimestamp>,
}

impl AckFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        // construct the type octet
        let mut frame_type = FRAME_FLAG_ACK;

        if self.ack_blocks.len() > 1 {
            frame_type |= 0b00100000
        }

        // TODO: calculate this more intelligently
        let largest_ack_length = 6;
        frame_type |= 0b00001100;

        // TODO: calculate this more intelligently
        let ack_block_length = 6;
        frame_type |= 0b00000011;

        write.write_u8(frame_type)?;

        // other fields
        if self.ack_blocks.len() > 1 {
            write.write_u8(
                cast::u8(self.ack_blocks.len())
                .expect("Too many ack blocks, count has to fit in 8 bits")
            )?;
        }
        write.write_u8(self.num_timestamps)?;
        write.write_uint::<BigEndian>(self.largest_acknowledged & 0x00FFFFFFFFFFFFFF, largest_ack_length)?;
        write.write_u16::<BigEndian>(self.ack_delay)?;

        if self.ack_blocks.is_empty() {
            panic!("The vector of ACK blocks cannot be empty");
        }
        match self.ack_blocks[0].gap {
            Some(_) => panic!("First ack block cannot have a gap"),
            _ => {},
        }
        write.write_uint::<BigEndian>(self.ack_blocks[0].block_length, ack_block_length)?;
        for ack_block in &self.ack_blocks[1..] {
            match ack_block.gap {
                Some(gap) => {
                    write.write_u8(gap)?;
                },
                None => panic!("Consequent ack blocks must have a gap"),
            }

            write.write_uint::<BigEndian>(ack_block.block_length, ack_block_length)?;
        }

        // TODO: checks
        for timestamp in &self.timestamps {
            match *timestamp {
                AckTimestamp::Delta(delta) => {
                    write.write_u8(delta)?;
                },
                AckTimestamp::FirstTimeStamp(ts) => {
                    write.write_u32::<BigEndian>(ts)?;
                },
                AckTimestamp::TimeSincePrevious(delta_ts) => {
                    write.write_u16::<BigEndian>(delta_ts)?;
                },
            }
        }
        
        Ok(())
    }
}
