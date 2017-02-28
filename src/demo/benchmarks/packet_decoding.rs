use std::io;
use std::io::Seek;
use std::time;

use quic::QUIC_VERSION;
use quic::endpoint::EndpointType;
use quic::frames;
use quic::packets;
use super::format_duration;

const ITERATION_COUNT: usize = 1000000;
const STREAM_DATA_SIZE: usize = 1000;


fn get_sample_packet_bytes() -> Vec<u8> {
    let mut write = io::Cursor::new(Vec::new());
    let stream_data = vec![42; STREAM_DATA_SIZE];

    let packet = packets::Packet::Regular(
        packets::RegularPacket {
            header: packets::PacketHeader {
                key_phase: true,
                packet_number_size: 4,
                multipath: true,

                connection_id: Some(0xABCDEF1234567890),
            },

            version: Some(QUIC_VERSION),
            packet_number: 0x1234567890ABCDEF,
            payload: packets::PacketPayload {
                frames: vec![
                    frames::Frame::Ping(frames::ping::PingFrame {}),
                    frames::Frame::Stream(frames::stream::StreamFrame {
                        stream_id: 1,
                        offset: 0 as u64,
                        stream_data: stream_data.clone(),
                        fin: false,
                    }),
                ],
            },
        }
    );
    packet.encode(&mut write).unwrap();

    write.into_inner()
}

pub fn run_benchmark() {
    info!("packet_decoding: started...");

    let mut read = io::Cursor::new(get_sample_packet_bytes());
    let mut bytes_total = 0;

    let start = time::Instant::now();
    for _ in 0..ITERATION_COUNT {
        let _ = packets::Packet::decode(&mut read, EndpointType::Server).unwrap();

        bytes_total += read.position();
        read.seek(io::SeekFrom::Start(0)).unwrap();
    }
    let elapsed = start.elapsed();

    info!(
        "packet_decoding: decoded {} packets ({} bytes total) in {}",
        ITERATION_COUNT,
        bytes_total,
        format_duration(elapsed),
    );
}
