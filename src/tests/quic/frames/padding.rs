use std::io;

use quic::frames::padding;


#[test]
fn test_encoding() {
    let frame = padding::PaddingFrame {};
    let mut write = io::Cursor::new(Vec::new());
    frame.encode(&mut write).unwrap();
    assert_eq!(
        write.get_ref(),
        &[
            0x00,
        ]
    );
}
