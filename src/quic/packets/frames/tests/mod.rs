mod ack;
mod blocked;
mod connection_close;
mod goaway;
mod padding;
mod ping;
mod rst_stream;
mod stop_waiting;
mod stream;
mod window_update;

use std::io;

use quic::errors::Error;
use quic::packets::frames;


#[test]
fn test_encoding() {
    let mut write = io::Cursor::new(Vec::new());

    let frame = frames::Frame::Ack(
        frames::ack::AckFrame {
            largest_acknowledged: 42,
            ack_delay: 32,

            first_ack_block_length: 64,
            extra_ack_blocks: Vec::new(),

            first_timestamp: None,
            extra_timestamps: Vec::new(),
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::Blocked(
        frames::blocked::BlockedFrame { stream_id: 42 }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::ConnectionClose(
        frames::connection_close::ConnectionCloseFrame {
            error_code: 42,
            reason_phrase: None,
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::GoAway(
        frames::goaway::GoAwayFrame {
            error_code: 42,
            last_good_stream_id: 32,
            reason_phrase: None,
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::Padding(
        frames::padding::PaddingFrame {}
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::Ping(
        frames::ping::PingFrame {}
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::RstStream(
        frames::rst_stream::RstStreamFrame {
            error_code: 42,
            stream_id: 32,
            final_offset: 64,
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::StopWaiting(
        frames::stop_waiting::StopWaitingFrame {
            least_acked_delta: 0x00001042DEADCAFE,
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::Stream(
        frames::stream::StreamFrame {
            stream_id: 42,
            offset: 32,
            stream_data: vec![0x68, 0x65, 0x6C, 0x6C, 0x6F],
            fin: false,
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    let frame = frames::Frame::WindowUpdate(
        frames::window_update::WindowUpdateFrame {
            stream_id: 42,
            byte_offset: 32,
        }
    );
    frame.encode(&mut write, 6, false).unwrap();

    assert_eq!(
        write.into_inner(),
        vec![
            // ack frame
            0x4F,
            0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,

            // blocked frame
            0x05,
            0x00, 0x00, 0x00, 0x2A,

            // connection close frame
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00,

            // goaway frame
            0x03,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00,

            // padding frame
            0x00,

            // ping frame
            0x07,

            // rst stream frame
            0x01,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,

            // stop waiting frame
            0x06,
            0x10, 0x42, 0xDE, 0xAD, 0xCA, 0xFE,

            // stream frame
            0xBF,
            0x00, 0x05,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
            0x68, 0x65, 0x6C, 0x6C, 0x6F,

            // window update frame
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
        ]
    );
}

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            // ack frame
            0x4F,
            0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x2A,
            0x00, 0x20,

            0x00, 0x00, 0x00, 0x00, 0x00, 0x40,

            // blocked frame
            0x05,
            0x00, 0x00, 0x00, 0x2A,

            // connection close frame
            0x02,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00,

            // goaway frame
            0x03,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00,

            // padding frame
            0x00,

            // ping frame
            0x07,

            // rst stream frame
            0x01,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x20,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,

            // stop waiting frame
            0x06,
            0x10, 0x42, 0xDE, 0xAD, 0xCA, 0xFE,

            // stream frame
            0xBF,
            0x00, 0x05,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
            0x68, 0x65, 0x6C, 0x6C, 0x6F,

            // window update frame
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
        ]
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::Ack(
            frames::ack::AckFrame {
                largest_acknowledged: 42,
                ack_delay: 32,

                first_ack_block_length: 64,
                extra_ack_blocks: Vec::new(),

                first_timestamp: None,
                extra_timestamps: Vec::new(),
            }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::Blocked(
            frames::blocked::BlockedFrame { stream_id: 42 }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::ConnectionClose(
            frames::connection_close::ConnectionCloseFrame {
                error_code: 42,
                reason_phrase: None,
            }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::GoAway(
            frames::goaway::GoAwayFrame {
                error_code: 42,
                last_good_stream_id: 32,
                reason_phrase: None,
            }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::Padding(
            frames::padding::PaddingFrame {}
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::Ping(
            frames::ping::PingFrame {}
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::RstStream(
            frames::rst_stream::RstStreamFrame {
                error_code: 42,
                stream_id: 32,
                final_offset: 64,
            }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::StopWaiting(
            frames::stop_waiting::StopWaitingFrame {
                least_acked_delta: 0x00001042DEADCAFE,
            }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::Stream(
            frames::stream::StreamFrame {
                stream_id: 42,
                offset: 32,
                stream_data: vec![0x68, 0x65, 0x6C, 0x6C, 0x6F],
                fin: false,
            }
        )
    );

    let frame = frames::Frame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        frames::Frame::WindowUpdate(
            frames::window_update::WindowUpdateFrame {
                stream_id: 42,
                byte_offset: 32,
            }
        )
    );

    match frames::Frame::decode(&mut read, 6) {
        Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => {},
        _ => assert!(false, "UnexpectedEof expected"),
    };

    let mut read = io::Cursor::new(
        vec![
            // incomplete window update frame
            0x04,
            0x00, 0x00, 0x00, 0x2A,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    );
    match frames::Frame::decode(&mut read, 6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };

    let mut read = io::Cursor::new(
        vec![
            // invalid frame type
            0x20,
        ]
    );
    match frames::Frame::decode(&mut read, 6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };
}
