use std::io;

use quic::errors::Error;
use quic::frames::stop_waiting;


#[test]
fn test_encoding() {
    let frame = stop_waiting::StopWaitingFrame {
        least_acked_delta: 0x00000000000000FE,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write, 1).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x06,
            0xFE,
        ]
    );

    let frame = stop_waiting::StopWaitingFrame {
        least_acked_delta: 0x000000000000CAFE,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write, 2).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x06,
            0xCA, 0xFE,
        ]
    );

    let frame = stop_waiting::StopWaitingFrame {
        least_acked_delta: 0x00000000DEADCAFE,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write, 4).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x06,
            0xDE, 0xAD, 0xCA, 0xFE,
        ]
    );

    let frame = stop_waiting::StopWaitingFrame {
        least_acked_delta: 0x00001042DEADCAFE,
    };
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write, 6).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x06,
            0x10, 0x42, 0xDE, 0xAD, 0xCA, 0xFE,
        ]
    );
}

#[test]
fn test_decoding() {
    let mut read = io::Cursor::new(
        vec![
            0x06,
            0xFE,
        ]
    );
    let frame = stop_waiting::StopWaitingFrame::decode(&mut read, 1).unwrap();
    assert_eq!(
        frame,
        stop_waiting::StopWaitingFrame {
            least_acked_delta: 0x00000000000000FE,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x06,
            0xCA, 0xFE,
        ]
    );
    let frame = stop_waiting::StopWaitingFrame::decode(&mut read, 2).unwrap();
    assert_eq!(
        frame,
        stop_waiting::StopWaitingFrame {
            least_acked_delta: 0x000000000000CAFE,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x06,
            0xDE, 0xAD, 0xCA, 0xFE,
        ]
    );
    let frame = stop_waiting::StopWaitingFrame::decode(&mut read, 4).unwrap();
    assert_eq!(
        frame,
        stop_waiting::StopWaitingFrame {
            least_acked_delta: 0x00000000DEADCAFE,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x06,
            0x10, 0x42, 0xDE, 0xAD, 0xCA, 0xFE,
        ]
    );
    let frame = stop_waiting::StopWaitingFrame::decode(&mut read, 6).unwrap();
    assert_eq!(
        frame,
        stop_waiting::StopWaitingFrame {
            least_acked_delta: 0x00001042DEADCAFE,
        }
    );

    let mut read = io::Cursor::new(
        vec![
            0x06,
            0x10, 0x42, 0xDE, 0xAD, 0xCA,
        ]
    );
    match stop_waiting::StopWaitingFrame::decode(&mut read, 6) {
        Err(Error::Decoding(..)) => {},
        _ => assert!(false, "Error expected"),
    };
}
