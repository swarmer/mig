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
            buffer: VecDeque::new(),
        }
    }

    pub fn add_data(&mut self, position: u64, data: &[u8]) -> Result<()> {
        let max_available_index = self.next_index + (self.capacity as u64) - 1;
        let max_arriving_index = position + (data.count() as u64) - 1;
        if (max_arriving_index > max_available_index) {
            return Err(Error::BufferOverflow);
        }

        let starting_buffer_index = position - self.next_index;
        // TODO
    }
}
