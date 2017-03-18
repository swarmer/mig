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

    outgoing_buffer: Vec<u8>,
    sent_offset: u64,
}

impl Stream {
    pub fn new(id: u32) -> Stream {
        Stream {
            id: id,
            outgoing_buffer: Vec::new(),
            sent_offset: 0,
            state: StreamState::Idle,
        }
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
