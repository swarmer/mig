use std::io;

use byteorder::{BigEndian, WriteBytesExt};
use cast;

use quic::errors::Result;
use quic::utils;


pub const FRAME_FLAG_ACK: u8 = 0b01000000;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ExtraAckBlock {
    pub gap: u8,
    pub block_count: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FirstAckTimestamp {
    pub delta_la: u8,
    pub delta_timestamp: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ExtraAckTimestamp {
    pub delta_la: u8,
    pub delta_timestamp: u16,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AckFrame {
    // header
    pub largest_acknowledged: u64,
    pub ack_delay: u16,

    // ack block section
    pub first_ack_block_count: u64,
    pub extra_ack_blocks: Vec<ExtraAckBlock>,

    // timestamp section
    pub first_timestamp: Option<FirstAckTimestamp>,
    pub extra_timestamps: Vec<ExtraAckTimestamp>,
}

impl AckFrame {
    pub fn encode(&self, write: &mut io::Write) -> Result<()> {
        // construct the type octet
        let mut frame_type = FRAME_FLAG_ACK;

        if !self.extra_ack_blocks.is_empty() {
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
        if !self.extra_ack_blocks.is_empty() {
            write.write_u8(
                cast::u8(self.extra_ack_blocks.len())
                .expect("Too many additional ack blocks, count has to fit in 8 bits")
            )?;
        }
        write.write_u8(
            cast::u8(
                if self.first_timestamp.is_some() { 1 } else { 0 } +
                self.extra_timestamps.len()
            )
            .expect("Too many timestamp blocks, count has to fit in 8 bits")
        )?;
        write.write_uint::<BigEndian>(
            utils::truncate_u64(self.largest_acknowledged, largest_ack_length),
            largest_ack_length,
        )?;
        write.write_u16::<BigEndian>(self.ack_delay)?;

        // ack block section
        write.write_uint::<BigEndian>(self.first_ack_block_count, ack_block_length)?;
        for ack_block in &self.extra_ack_blocks[..] {
            write.write_u8(ack_block.gap)?;
            write.write_uint::<BigEndian>(ack_block.block_count, ack_block_length)?;
        }

        // timestamp section
        if self.first_timestamp.is_none() && !self.extra_timestamps.is_empty() {
            panic!("Must have first timestamp before extra timestamps");
        }
        if let Some(FirstAckTimestamp { delta_la, delta_timestamp }) = self.first_timestamp {
            write.write_u8(delta_la)?;
            write.write_u32::<BigEndian>(delta_timestamp)?;
        }
        for timestamp in &self.extra_timestamps {
            write.write_u8(timestamp.delta_la)?;
            write.write_u16::<BigEndian>(timestamp.delta_timestamp)?;
        }
        
        Ok(())
    }
}
