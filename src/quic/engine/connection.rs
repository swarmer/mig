use std::cmp::min;
use std::net;

use quic::QUIC_VERSION;
use quic::endpoint_role::EndpointRole;
use quic::errors::Result;
use quic::packets::frames::Frame;
use quic::packets::frames::stream;
use quic::packets;
use super::stream::Stream;


const MAX_DATA_SIZE: usize = 1000;

#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    id: u64,
    endpoint_role: EndpointRole,
    peer_address: net::SocketAddr,
    streams: Vec<Stream>,
}

impl Connection {
    pub fn new(id: u64, endpoint_role: EndpointRole, peer_address: net::SocketAddr) -> Connection {
        Connection {
            id: id,
            endpoint_role: endpoint_role,
            peer_address: peer_address,
            streams: Vec::new(),
        }
    }

    pub fn write(&mut self, stream_id: u32, buf: &[u8]) -> Result<()> {
        self.extend_streams(stream_id);
        let ref mut stream = self.streams[stream_id as usize];
        stream.extend_buf(buf);

        Ok(())
    }

    pub fn drain_outgoing_packets(&mut self) -> Vec<packets::Packet> {
        let mut packets = vec![];
        let mut frames = vec![];
        let mut data_length = 0;

        for stream in &mut self.streams {
            let (mut sent_offset, stream_buffer) = stream.drain_outgoing_buffer();
            let mut stream_buffer = &stream_buffer[..];

            while !stream_buffer.is_empty() {
                let can_fit = MAX_DATA_SIZE - data_length;
                let will_fit = min(can_fit, stream_buffer.len());
                if will_fit > 0 {
                    frames.push(Frame::Stream(
                        stream::StreamFrame {
                            stream_id: stream.id,
                            offset: sent_offset,
                            stream_data: Vec::from(&stream_buffer[..will_fit]),
                            fin: false,
                        }
                    ));

                    stream_buffer = &stream_buffer[will_fit..];
                    sent_offset += will_fit as u64;
                }

                if !stream_buffer.is_empty() {
                    packets.push(Self::create_stream_packet(self.id, frames));
                    frames = vec![];
                    data_length = 0;
                }
            }
        }

        if !frames.is_empty() {
            packets.push(Self::create_stream_packet(self.id, frames));
            frames = vec![];
            data_length = 0;
        }

        packets
    }

    pub fn peer_address(&self) -> net::SocketAddr {
        self.peer_address
    }

    fn create_stream_packet(connection_id: u64, frames: Vec<Frame>) -> packets::Packet {
        let header = packets::PacketHeader {
            key_phase: false,
            packet_number_size: 4,
            multipath: false,

            connection_id: Some(connection_id),
        };

        packets::Packet::Regular(packets::RegularPacket {
            header: header,

            version: Some(QUIC_VERSION),
            packet_number: 0,
            payload: packets::PacketPayload {
                frames: frames,
            },
        })
    }

    fn extend_streams(&mut self, new_max_id: u32) {
        let next_max_id = self.streams.len() as u32;
        let additional_count = new_max_id - next_max_id + 1;
        if additional_count <= 0 {
            return;
        }

        for stream_id in next_max_id..(new_max_id + 1) {
            self.streams.push(Stream::new(stream_id));
        }
    }
}
