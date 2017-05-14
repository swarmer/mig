use std::cmp::min;

use quic::errors::Result;
use super::stream_buffer::StreamBuffer;


pub const INCOMING_BUFFER_SIZE: usize = 100 * 1024;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StreamState {
    Idle,
    Open,
    RemoteClosed,
    LocalClosed,
    Closed,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Stream {
    pub id: u32,
    pub state: StreamState,
    pub fin_sent: bool,

    incoming_buffer: StreamBuffer,
    prev_maximum_data: u64,
    fin_offset: u64,

    outgoing_buffer: Vec<u8>,
    pub max_outgoing_data: u64,
    next_outgoing_offset: u64,
}

impl Stream {
    pub fn new(id: u32) -> Stream {
        Stream {
            id: id,
            state: StreamState::Idle,
            fin_sent: false,

            incoming_buffer: StreamBuffer::new(INCOMING_BUFFER_SIZE),
            prev_maximum_data: 0,
            fin_offset: 0,

            outgoing_buffer: vec![],
            max_outgoing_data: INCOMING_BUFFER_SIZE as u64,
            next_outgoing_offset: 0,
        }
    }

    pub fn data_available(&self) -> bool {
        trace!(
            "data_available, readable: {:?}, state: {:?}, next_index: {:?}, fin_offset: {:?}",
            self.incoming_buffer.is_readable(),
            self.state,
            self.incoming_buffer.next_index,
            self.fin_offset,
        );
        self.incoming_buffer.is_readable() || (
            [StreamState::RemoteClosed, StreamState::Closed].contains(&self.state) &&
            self.incoming_buffer.next_index == self.fin_offset
        )
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_size = self.incoming_buffer.pull_data(buf);
        debug!("read called, size: {}", read_size);
        Ok(read_size)
    }

    pub fn finalize_outgoing(&mut self) {
        self.state = match self.state {
            StreamState::Idle => StreamState::LocalClosed,
            StreamState::Open => StreamState::LocalClosed,
            StreamState::RemoteClosed => StreamState::Closed,
            StreamState::LocalClosed => StreamState::LocalClosed,
            StreamState::Closed => StreamState::Closed,
        };
    }

    pub fn outgoing_fin_offset(&self) -> u64 {
        self.next_outgoing_offset + self.outgoing_buffer.len() as u64
    }

    pub fn finalize_incoming(&mut self, offset: u64) {
        self.fin_offset = offset;

        self.state = match self.state {
            StreamState::Idle => StreamState::RemoteClosed,
            StreamState::Open => StreamState::RemoteClosed,
            StreamState::RemoteClosed => StreamState::RemoteClosed,
            StreamState::LocalClosed => StreamState::Closed,
            StreamState::Closed => StreamState::Closed,
        };
    }

    pub fn extend_outgoing_buf(&mut self, buf: &[u8]) {
        self.outgoing_buffer.extend(buf);
        self.state = StreamState::Open;
    }

    pub fn extend_incoming_buf(&mut self, offset: u64, buf: &[u8]) -> Result<()> {
        if buf.is_empty() {
            return Ok(())
        }

        self.state = match self.state {
            StreamState::Idle | StreamState::Open => StreamState::Open,
            StreamState::RemoteClosed => StreamState::RemoteClosed,
            StreamState::LocalClosed => StreamState::LocalClosed,
            StreamState::Closed => StreamState::Closed,
        };

        self.incoming_buffer.add_data(offset, buf)
    }

    pub fn new_maximum_data(&mut self) -> Option<u64> {
        let maximum_data = self.incoming_buffer.maximum_accepted_offset() + 1;

        if maximum_data != self.prev_maximum_data {
            self.prev_maximum_data = maximum_data;
            Some(maximum_data)
        } else {
            None
        }
    }

    pub fn drain_outgoing_buffer(&mut self) -> (u64, Vec<u8>) {
        let can_send = (self.max_outgoing_data - self.next_outgoing_offset) as usize;
        let will_send = min(can_send, self.outgoing_buffer.len());

        let next_outgoing_offset = self.next_outgoing_offset;
        self.next_outgoing_offset += will_send as u64;

        (next_outgoing_offset, self.outgoing_buffer.drain(..will_send).collect())
    }
}
