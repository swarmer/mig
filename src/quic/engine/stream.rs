use std::cmp::min;

use quic::errors::Result;
use quic::packets::frames::stream;


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

    incoming_buffer: Vec<u8>,

    outgoing_buffer: Vec<u8>,
    sent_offset: u64,
}

impl Stream {
    pub fn new(id: u32) -> Stream {
        Stream {
            id: id,
            incoming_buffer: vec![],
            outgoing_buffer: vec![],
            sent_offset: 0,
            state: StreamState::Idle,
        }
    }

    pub fn data_available(&self) -> bool {
        !self.incoming_buffer.is_empty()
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_size = min(buf.len(), self.incoming_buffer.len());
        buf.copy_from_slice(&self.incoming_buffer[..read_size]);
        self.incoming_buffer.drain(..read_size);

        Ok(read_size)
    }

    pub fn extend_buf(&mut self, buf: &[u8]) {
        self.outgoing_buffer.extend(buf);
        self.state = StreamState::Open;
    }

    pub fn drain_outgoing_buffer(&mut self) -> (u64, Vec<u8>) {
        let sent_offset = self.sent_offset;
        self.sent_offset += self.outgoing_buffer.len() as u64;

        (sent_offset, self.outgoing_buffer.drain(..).collect())
    }
}
