use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cast;

use quic::errors::Result;
use quic::utils::{map_unexpected_eof, truncate_u64};


pub const FLAG_ACK: u8 = 0b01000000;
pub const FLAG_EXTRA_ACK_BLOCKS: u8 = 0b00100000;

pub const MASK_ACK: u8 = 0b11000000;
pub const MASK_LARGEST_ACK_SIZE: u8 = 0b00001100;
pub const MASK_ACK_BLOCK_SIZE: u8 = 0b00000011;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ExtraAckBlock {
    pub gap: u8,
    pub block_length: u64,
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
    pub first_ack_block_length: u64,
    pub extra_ack_blocks: Vec<ExtraAckBlock>,

    // timestamp section
    pub first_timestamp: Option<FirstAckTimestamp>,
    pub extra_timestamps: Vec<ExtraAckTimestamp>,
}

impl AckFrame {
    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        // construct the type octet
        let mut frame_type = FLAG_ACK;

        if !self.extra_ack_blocks.is_empty() {
            frame_type |= FLAG_EXTRA_ACK_BLOCKS;
        }

        // TODO: calculate this more intelligently
        let largest_ack_size = 6;
        frame_type |= 0b00001100;

        // TODO: calculate this more intelligently
        let ack_block_size = 6;
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
            truncate_u64(self.largest_acknowledged, largest_ack_size),
            largest_ack_size,
        )?;
        write.write_u16::<BigEndian>(self.ack_delay)?;

        // ack block section
        write.write_uint::<BigEndian>(self.first_ack_block_length, ack_block_size)?;
        for ack_block in &self.extra_ack_blocks[..] {
            write.write_u8(ack_block.gap)?;
            write.write_uint::<BigEndian>(ack_block.block_length, ack_block_size)?;
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

    pub fn decode<R: io::Read>(read: &mut R) -> Result<AckFrame> {
        // extract type octet data
        let frame_type = read.read_u8()?;
        assert!((frame_type & MASK_ACK) == FLAG_ACK);

        let has_extra_ack_blocks = (frame_type & FLAG_EXTRA_ACK_BLOCKS) != 0;

        let largest_ack_size = match (frame_type & MASK_LARGEST_ACK_SIZE) >> 2 {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            0b11 => 6,
            _ => unreachable!(),
        };

        let ack_block_size = match frame_type & MASK_ACK_BLOCK_SIZE {
            0b00 => 1,
            0b01 => 2,
            0b10 => 4,
            0b11 => 6,
            _ => unreachable!(),
        };

        // other fields
        let extra_ack_block_count = if has_extra_ack_blocks {
            read.read_u8().map_err(map_unexpected_eof)? as usize
        } else {
            0
        };

        let timestamp_count = read.read_u8().map_err(map_unexpected_eof)? as usize;

        let largest_acknowledged = 
            read.read_uint::<BigEndian>(largest_ack_size)
            .map_err(map_unexpected_eof)?;
        
        let ack_delay = 
            read.read_u16::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        
        // ack block section
        let first_ack_block_length = 
            read.read_uint::<BigEndian>(ack_block_size)
            .map_err(map_unexpected_eof)?;
        
        let mut extra_ack_blocks = Vec::new();
        for _ in 0..extra_ack_block_count {
            let gap = read.read_u8().map_err(map_unexpected_eof)?;
            let block_length = 
                read.read_uint::<BigEndian>(ack_block_size)
                .map_err(map_unexpected_eof)?;

            extra_ack_blocks.push(ExtraAckBlock { gap: gap, block_length: block_length });
        }

        // timestamp section
        let first_timestamp = if timestamp_count > 0 {
            let delta_la = read.read_u8().map_err(map_unexpected_eof)?;
            let delta_timestamp = read.read_u32::<BigEndian>().map_err(map_unexpected_eof)?;
            Some(FirstAckTimestamp { delta_la: delta_la, delta_timestamp: delta_timestamp })
        } else {
            None
        };

        let mut extra_timestamps = Vec::new();
        if timestamp_count > 1 {
            for _ in 0..(timestamp_count - 1) {
                let delta_la = read.read_u8().map_err(map_unexpected_eof)?;
                let delta_timestamp = read.read_u16::<BigEndian>().map_err(map_unexpected_eof)?;
                extra_timestamps.push(ExtraAckTimestamp { delta_la: delta_la, delta_timestamp: delta_timestamp })
            }
        }

        Ok(AckFrame {
            largest_acknowledged: largest_acknowledged,
            ack_delay: ack_delay,

            first_ack_block_length: first_ack_block_length,
            extra_ack_blocks: extra_ack_blocks,

            first_timestamp: first_timestamp,
            extra_timestamps: extra_timestamps,
        })
    }
}
