pub mod ack;
pub mod blocked;
pub mod connection_close;
pub mod goaway;
pub mod padding;
pub mod ping;
pub mod rst_stream;
pub mod stop_waiting;
pub mod stream;
pub mod window_update;

use std::io;

use quic::errors::Result;


#[derive(Clone, Debug, PartialEq)]
pub enum Frame {
    Ack(ack::AckFrame),
    Blocked(blocked::BlockedFrame),
    ConnectionClose(connection_close::ConnectionCloseFrame),
    GoAway(goaway::GoAwayFrame),
    Padding(padding::PaddingFrame),
    Ping(ping::PingFrame),
    RstStream(rst_stream::RstStreamFrame),
    StopWaiting(stop_waiting::StopWaitingFrame),
    Stream(stream::StreamFrame),
    WindowUpdate(window_update::WindowUpdateFrame),
}

impl Frame {
    pub fn encode(&self, write: &mut io::Write, packet_number_size: usize) -> Result<()> {
        match *self {
            Frame::Ack(ref ack_frame) => ack_frame.encode(write),
            Frame::Blocked(ref blocked_frame) => blocked_frame.encode(write),
            Frame::ConnectionClose(ref connection_close_frame) => connection_close_frame.encode(write),
            Frame::GoAway(ref goaway_frame) => goaway_frame.encode(write),
            Frame::Padding(ref padding_frame) => padding_frame.encode(write),
            Frame::Ping(ref ping_frame) => ping_frame.encode(write),
            Frame::RstStream(ref rst_stream_frame) => rst_stream_frame.encode(write),
            Frame::StopWaiting(ref stop_waiting_frame) => stop_waiting_frame.encode(write, packet_number_size),
            Frame::Stream(ref stream_frame) => stream_frame.encode(write, false),
            Frame::WindowUpdate(ref window_update_frame) => window_update_frame.encode(write),
        }
    }
}
