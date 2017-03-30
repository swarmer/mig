use std::cmp::min;

use quic::errors::Result;


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
            fin_sent: false,
        }
    }

    pub fn data_available(&self) -> bool {
        !self.incoming_buffer.is_empty() ||
            [StreamState::RemoteClosed, StreamState::Closed].contains(&self.state)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_size = min(buf.len(), self.incoming_buffer.len());
        let buf = &mut buf[..read_size];
        buf.copy_from_slice(&self.incoming_buffer[..read_size]);
        self.incoming_buffer.drain(..read_size);

        debug!("read called, size: {}, incoming_buffer.len(): {}", read_size, self.incoming_buffer.len());
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

    pub fn finalize_incoming(&mut self) {
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

    pub fn extend_incoming_buf(&mut self, buf: &[u8]) {
        self.incoming_buffer.extend(buf);
    }

    pub fn drain_outgoing_buffer(&mut self) -> (u64, Vec<u8>) {
        let sent_offset = self.sent_offset;
        self.sent_offset += self.outgoing_buffer.len() as u64;

        (sent_offset, self.outgoing_buffer.drain(..).collect())
    }
}
