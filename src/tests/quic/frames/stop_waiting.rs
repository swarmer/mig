use std::io;

use quic::frames::stop_waiting;


#[test]
fn test_encoding() {
    let frame = stop_waiting::StopWaitingFrame {
        least_acked_delta: 0x11121042DEADCAFE,
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
        least_acked_delta: 0x11121042DEADCAFE,
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
        least_acked_delta: 0x11121042DEADCAFE,
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
        least_acked_delta: 0x11121042DEADCAFE,
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
