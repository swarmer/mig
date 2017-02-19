use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use cast;


pub const FRAME_PADDING: u8 = 0x00;
pub const FRAME_RST_STREAM: u8 = 0x01;
pub const FRAME_CONNECTION_CLOSE: u8 = 0x02;
pub const FRAME_GOAWAY: u8 = 0x03;
pub const FRAME_WINDOW_UPDATE: u8 = 0x04;
pub const FRAME_BLOCKED: u8 = 0x05;
pub const FRAME_STOP_WAITING: u8 = 0x06;
pub const FRAME_PING: u8 = 0x07;

pub const FRAME_FLAG_ACK: u8 = 0b01000000;
pub const FRAME_FLAG_STREAM: u8 = 0b10000000;

pub struct AckBlock {
    pub gap: Option<u8>,
    pub block_length: u64,
}

pub enum AckTimestamp {
    Delta(u8),
    FirstTimeStamp(u32),
    TimeSincePrevious(u16),
}

pub enum Frame {
    Padding,
    RstStream {
        error_code: u32,
        stream_id: u32,
        final_offset: u64,
    },
    ConnectionClose {
        error_code: u32,
        reason_phrase: Option<String>,
    },
    GoAway {
        error_code: u32,
        last_good_stream_id: u32,
        reason_phrase: Option<String>,
    },
    WindowUpdate {
        stream_id: u32,
        byte_offset: u64,
    },
    Blocked {
        stream_id: u32,
    },
    StopWaiting {
        least_acked_delta: u64,
    },
    Ping,
    Ack {
        num_timestamps: u8,
        largest_acknowledged: u64,
        ack_delay: u16,
        ack_blocks: Vec<AckBlock>,
        timestamps: Vec<AckTimestamp>,
    },
    Stream {
        stream_id: u32,
        offset: u64,
        stream_data: Vec<u8>,
        fin: bool,
    },
}

impl Frame {
    pub fn encode(&self, write: &mut io::Write, packet_number_size: usize) -> io::Result<()> {
        match *self {
            Frame::Padding => {
                write.write_u8(FRAME_PADDING)?;

                Ok(())
            },
            Frame::RstStream { error_code, stream_id, final_offset } => {
                write.write_u8(FRAME_RST_STREAM)?;

                write.write_u32::<BigEndian>(error_code)?;
                write.write_u32::<BigEndian>(stream_id)?;
                write.write_u64::<BigEndian>(final_offset)?;

                Ok(())
            },
            Frame::ConnectionClose { error_code, ref reason_phrase } => {
                write.write_u8(FRAME_CONNECTION_CLOSE)?;

                write.write_u32::<BigEndian>(error_code)?;

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
            },
            Frame::GoAway { error_code, last_good_stream_id, ref reason_phrase } => {
                write.write_u8(FRAME_GOAWAY)?;

                write.write_u32::<BigEndian>(error_code)?;
                write.write_u32::<BigEndian>(last_good_stream_id)?;

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
            },
            Frame::WindowUpdate { stream_id, byte_offset } => {
                write.write_u8(FRAME_WINDOW_UPDATE)?;

                write.write_u32::<BigEndian>(stream_id)?;
                write.write_u64::<BigEndian>(byte_offset)?;

                Ok(())
            },
            Frame::Blocked { stream_id } => {
                write.write_u8(FRAME_BLOCKED)?;

                write.write_u32::<BigEndian>(stream_id)?;

                Ok(())
            },
            Frame::StopWaiting { least_acked_delta } => {
                write.write_u8(FRAME_STOP_WAITING)?;

                match packet_number_size {
                    1 | 2 | 4 | 6 => {
                        write.write_uint::<BigEndian>(least_acked_delta, packet_number_size)?;
                    },
                    _ => panic!("Invalid packet number size: {}", packet_number_size),
                }

                Ok(())
            },
            Frame::Ping => {
                write.write_u8(FRAME_PING)?;

                Ok(())
            },
            Frame::Ack { num_timestamps, largest_acknowledged, ack_delay, ref ack_blocks, ref timestamps } => {
                // construct the type octet
                let mut frame_type = FRAME_FLAG_ACK;

                if ack_blocks.len() > 1 {
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
                if ack_blocks.len() > 1 {
                    write.write_u8(
                        cast::u8(ack_blocks.len())
                        .expect("Too many ack blocks, count has to fit in 8 bits")
                    )?;
                }
                write.write_u8(num_timestamps)?;
                write.write_uint::<BigEndian>(largest_acknowledged & 0x00FFFFFFFFFFFFFF, largest_ack_length)?;
                write.write_u16::<BigEndian>(ack_delay)?;

                if ack_blocks.is_empty() {
                    panic!("The vector of ACK blocks cannot be empty");
                }
                match ack_blocks[0].gap {
                    Some(_) => panic!("First ack block cannot have a gap"),
                    _ => {},
                }
                write.write_uint::<BigEndian>(ack_blocks[0].block_length, ack_block_length)?;
                for ack_block in &ack_blocks[1..] {
                    match ack_blocks[0].gap {
                        Some(gap) => {
                            write.write_u8(gap)?;
                        },
                        None => panic!("Consequent ack blocks must have a gap"),
                    }

                    write.write_uint::<BigEndian>(ack_block.block_length, ack_block_length)?;
                }

                // TODO: checks
                for timestamp in timestamps {
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
            },
            Frame::Stream { stream_id, offset, ref stream_data, fin } => {
                // construct the type octet
                let mut frame_type = FRAME_FLAG_STREAM;

                if fin {
                    frame_type |= 0b01000000
                }

                // TODO: exclude data length sometimes?
                let has_data_length = true;
                if has_data_length {
                    frame_type |= 0b00100000;
                }

                // TODO: calculate this more intelligently
                let offset_length = 8;
                frame_type |= 0b00011100;

                // TODO: calculate this more intelligently
                let stream_id_length = 4;
                frame_type |= 0b00000011;

                write.write_u8(frame_type)?;

                // other fields
                if has_data_length {
                    write.write_u16::<BigEndian>(
                        cast::u16(stream_data.len())
                        .expect("Stream data too big, size has to fit in 16 bits")
                    )?;
                }
                write.write_uint::<BigEndian>(stream_id as u64, stream_id_length)?;
                write.write_uint::<BigEndian>(offset, offset_length)?;
                write.write_all(&stream_data[..])?;

                Ok(())
            },
        }
    }

    pub fn decode(read: &mut io::Read) -> Frame {
        unimplemented!()
    }
}
