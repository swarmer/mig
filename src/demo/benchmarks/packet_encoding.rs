use std::io;
use std::io::Seek;
use std::time;

use quic::frames;
use quic::packets;
use super::format_duration;

const ITERATION_COUNT: usize = 1000000;
const STREAM_DATA_SIZE: usize = 1000;


pub fn run_benchmark() {
    info!("packet_encoding: started...");

    let mut write = io::Cursor::new(Vec::with_capacity(2 * STREAM_DATA_SIZE));
    let stream_data = vec![42; STREAM_DATA_SIZE];

    let mut bytes_total: usize = 0;
    let start = time::Instant::now();
    for i in 0..ITERATION_COUNT {
        let packet = packets::Packet::Regular(
            packets::RegularPacket {
                header: packets::PacketHeader {
                    key_phase: true,
                    packet_number_size: 4,
                    multipath: true,

                    connection_id: Some(0xABCDEF1234567890),
                },

                version: Some(0x12345678),
                packet_number: 0x1234567890ABCDEF,
                payload: packets::PacketPayload {
                    frames: vec![
                        frames::Frame::Ping(frames::ping::PingFrame {}),
                        frames::Frame::Stream(frames::stream::StreamFrame {
                            stream_id: 1,
                            offset: (i * STREAM_DATA_SIZE) as u64,
                            stream_data: stream_data.clone(),
                            fin: false,
                        }),
                    ],
                },
            }
        );
        packet.encode(&mut write).unwrap();

        bytes_total += write.get_ref().len();
        write.seek(io::SeekFrom::Start(0)).unwrap();
    }
    let elapsed = start.elapsed();

    info!(
        "packet_encoding: encoded {} packets ({} bytes total) in {}",
        ITERATION_COUNT,
        bytes_total,
        format_duration(elapsed),
    );
}
