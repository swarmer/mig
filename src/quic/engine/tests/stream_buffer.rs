use quic::engine::stream_buffer::StreamBuffer;
use quic::errors::Error;


#[test]
fn test_add_data() {
    let mut buffer = StreamBuffer::new(100);
    let mut buf = [0, 0];

    let data = [1, 2, 3];
    buffer.add_data(0, &data).unwrap();

    assert_eq!(buffer.pull_data(&mut buf), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(buffer.pull_data(&mut buf), 1);
    assert_eq!(buf, [3, 2]);

    let data = [5, 6, 7];
    buffer.add_data(4, &data).unwrap();

    assert_eq!(buffer.pull_data(&mut buf), 0);
    assert_eq!(buf, [3, 2]);

    let data = [4];
    buffer.add_data(3, &data).unwrap();

    assert_eq!(buffer.pull_data(&mut buf), 2);
    assert_eq!(buf, [4, 5]);

    assert_eq!(buffer.pull_data(&mut buf), 2);
    assert_eq!(buf, [6, 7]);

    assert_eq!(buffer.pull_data(&mut buf), 0);
    assert_eq!(buf, [6, 7]);

    assert_eq!(buffer.pull_data(&mut buf), 0);
    assert_eq!(buf, [6, 7]);
}


#[test]
fn test_overflow() {
    let mut buffer = StreamBuffer::new(3);
    let data = [1, 2];
    buffer.add_data(0, &data).unwrap();

    let data = [3, 4];
    match buffer.add_data(2, &data) {
        Err(Error::BufferOverflow) => {},
        _ => assert!(false, "Buffer overflow expected"),
    };
}


#[test]
fn test_mismatch() {
    let mut buffer = StreamBuffer::new(3);
    let data = [1, 2];
    buffer.add_data(0, &data).unwrap();

    let data = [3, 4];
    match buffer.add_data(0, &data) {
        Err(Error::InvalidData(..)) => {},
        _ => assert!(false, "Buffer mismatch expected"),
    };
}
