use quic::errors::Result;
use super::stream_buffer::StreamBuffer;


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

    outgoing_buffer: Vec<u8>,
    sent_offset: u64,
}

impl Stream {
    pub fn new(id: u32) -> Stream {
        Stream {
            id: id,
            incoming_buffer: StreamBuffer::new(10000),
            outgoing_buffer: vec![],
            sent_offset: 0,
            state: StreamState::Idle,
            fin_sent: false,
        }
    }

    pub fn data_available(&self) -> bool {
        self.incoming_buffer.is_readable() ||
            [StreamState::RemoteClosed, StreamState::Closed].contains(&self.state)
            // TODO: wat?
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

    pub fn extend_incoming_buf(&mut self, offset: u64, buf: &[u8]) -> Result<()> {
        self.incoming_buffer.add_data(offset, buf)
    }

    pub fn drain_outgoing_buffer(&mut self) -> (u64, Vec<u8>) {
        let sent_offset = self.sent_offset;
        self.sent_offset += self.outgoing_buffer.len() as u64;

        (sent_offset, self.outgoing_buffer.drain(..).collect())
    }
}
