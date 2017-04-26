use std::cmp::min;
use std::collections::VecDeque;

use quic::errors::{Error, Result};


pub struct StreamBuffer {
    capacity: usize,
    next_index: u64,
    buffer: VecDeque<Option<u8>>,
}

impl StreamBuffer {
    pub fn new(capacity: usize) -> StreamBuffer {
        StreamBuffer {
            capacity: capacity,
            next_index: 0,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn add_data(&mut self, position: u64, data: &[u8]) -> Result<()> {
        // check that the buffer will not overflow
        let max_available_index = self.next_index + (self.capacity as u64) - 1;
        let max_arriving_index = position + (data.len() as u64) - 1;
        if max_arriving_index > max_available_index {
            return Err(Error::BufferOverflow);
        }

        // check that there's no mismatch with the existing data
        let starting_buffer_index = (position - self.next_index) as usize;
        let overlapping_count = min(
            data.len(),
            (self.buffer.len() - starting_buffer_index),
        );
        for i in 0..overlapping_count {
            let buffer_element = &mut self.buffer[starting_buffer_index + i];

            match *buffer_element {
                Some(existing_byte) => {
                    if existing_byte != data[i] {
                        warn!(
                            "Incorrect byte at index {} + {} (got {}, expected {})",
                            position, i, data[i], existing_byte,
                        );
                        return Err(
                            Error::InvalidData(String::from("Mismatch with bytes already in buffer"))
                        );
                    }
                },
                None => {
                    *buffer_element = Some(data[i]);
                }
            }
        }

        // append the rest of new data to the buffer
        self.buffer.extend(
            data[overlapping_count..]
            .iter()
            .map(|&b| Some(b))
        );

        Ok(())
    }
}
