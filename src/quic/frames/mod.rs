pub mod ack;
pub mod blocked;
pub mod connection_close;
pub mod goaway;
pub mod padding;
pub mod ping;
pub mod rst_stream;
pub mod stop_waiting;
pub mod stream;
mod utils;
pub mod window_update;

use std::io;

use byteorder::{ReadBytesExt};

use quic::errors::{Error, Result};
use quic::frames::utils::check_packet_number_size;


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
    pub fn encode(&self, write: &mut io::Write, packet_number_size: usize, last_frame: bool) -> Result<()> {
        check_packet_number_size(packet_number_size);

        match *self {
            Frame::Ack(ref ack_frame) => ack_frame.encode(write),
            Frame::Blocked(ref blocked_frame) => blocked_frame.encode(write),
            Frame::ConnectionClose(ref connection_close_frame) => connection_close_frame.encode(write),
            Frame::GoAway(ref goaway_frame) => goaway_frame.encode(write),
            Frame::Padding(ref padding_frame) => padding_frame.encode(write),
            Frame::Ping(ref ping_frame) => ping_frame.encode(write),
            Frame::RstStream(ref rst_stream_frame) => rst_stream_frame.encode(write),
            Frame::StopWaiting(ref stop_waiting_frame) => stop_waiting_frame.encode(write, packet_number_size),
            Frame::Stream(ref stream_frame) => stream_frame.encode(write, last_frame),
            Frame::WindowUpdate(ref window_update_frame) => window_update_frame.encode(write),
        }
    }

    pub fn decode<R>(read: &mut R, packet_number_size: usize) -> Result<Frame>
            where R: io::Read + io::Seek {
        check_packet_number_size(packet_number_size);

        let frame_type = read.read_u8()?;
        read.seek(io::SeekFrom::Current(-1)).unwrap();

        match frame_type {
            blocked::FRAME_BLOCKED =>
                Ok(Frame::Blocked(blocked::BlockedFrame::decode(read)?)),
            connection_close::FRAME_CONNECTION_CLOSE =>
                Ok(Frame::ConnectionClose(connection_close::ConnectionCloseFrame::decode(read)?)),
            goaway::FRAME_GOAWAY =>
                Ok(Frame::GoAway(goaway::GoAwayFrame::decode(read)?)),
            padding::FRAME_PADDING =>
                Ok(Frame::Padding(padding::PaddingFrame::decode(read)?)),
            ping::FRAME_PING =>
                Ok(Frame::Ping(ping::PingFrame::decode(read)?)),
            rst_stream::FRAME_RST_STREAM =>
                Ok(Frame::RstStream(rst_stream::RstStreamFrame::decode(read)?)),
            stop_waiting::FRAME_STOP_WAITING =>
                Ok(Frame::StopWaiting(stop_waiting::StopWaitingFrame::decode(read, packet_number_size)?)),
            window_update::FRAME_WINDOW_UPDATE =>
                Ok(Frame::WindowUpdate(window_update::WindowUpdateFrame::decode(read)?)),
            other_type => {
                if (other_type & ack::FRAME_MASK_ACK) == ack::FRAME_FLAG_ACK {
                    Ok(Frame::Ack(ack::AckFrame::decode(read)?))
                } else if (other_type & stream::FRAME_FLAG_STREAM) != 0 {
                    Ok(Frame::Stream(stream::StreamFrame::decode(read)?))
                } else {
                    Err(Error::Decoding(String::from("Invalid frame type")))
                }
            },
        }
    }
}
