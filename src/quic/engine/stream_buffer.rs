use std::cmp::min;
use std::collections::VecDeque;

use quic::errors::{Error, Result};


#[derive(Clone, Debug, PartialEq)]
pub struct StreamBuffer {
    capacity: usize,
    pub next_index: u64,
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

    pub fn add_data(&mut self, offset: u64, data: &[u8]) -> Result<()> {
        // check that the buffer will not overflow
        let max_available_index = self.maximum_accepted_offset();
        let max_arriving_index = offset + (data.len() as u64) - 1;
        if max_arriving_index > max_available_index {
            error!(
                "Overflow, next_index: {:?}, capacity: {:?}, offset: {:?}, data_len: {:?}",
                self.next_index,
                self.capacity,
                offset,
                data.len(),
            );
            return Err(Error::BufferOverflow);
        }

        // check that we are not writing data that's already been read
        if offset < self.next_index {
            return Err(Error::InvalidData(String::from("Stream data already delivered")));
        }

        // check that there's no mismatch with the existing data
        let starting_buffer_index = (offset - self.next_index) as usize;
        self.extend_buffer(starting_buffer_index);
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
                            offset, i, data[i], existing_byte,
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

    pub fn pull_data(&mut self, buf: &mut [u8]) -> usize {
        let possible_size = min(buf.len(), self.buffer.len());
        let mut actual_size = 0;
        for i in 0..possible_size {
            match self.buffer[i] {
                Some(b) => {
                    buf[i] = b;
                    actual_size += 1;
                },
                None => { break; },
            }
        }

        self.buffer.drain(..actual_size);
        self.next_index += actual_size as u64;

        actual_size
    }

    pub fn is_readable(&self) -> bool {
        self.buffer.len() > 0 && self.buffer[0].is_some()
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for item in &self.buffer {
            if item.is_some() {
                empty = false;
                break;
            }
        }

        empty
    }

    pub fn maximum_accepted_offset(&self) -> u64 {
        self.next_index + (self.capacity as u64) - 1
    }

    fn extend_buffer(&mut self, starting_buffer_index: usize) {
        for _ in self.buffer.len()..starting_buffer_index {
            self.buffer.push_back(None);
        }
    }
}
