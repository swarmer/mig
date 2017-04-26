use super::super::stream_buffer::StreamBuffer;


#[test]
fn lidl_test() {
    let mut buffer = StreamBuffer::new(100);
    let data: [u8; 3] = [1, 2, 3];
    buffer.add_data(0, &data).unwrap();
}
