use std::cmp::min;
use std::net;

use quic::endpoint_role::EndpointRole;
use quic::errors::Result;
use quic::packets::frames::Frame;
use quic::packets::frames::stream;
use quic::packets;
use super::stream::{Stream, StreamState};


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
        stream.extend_outgoing_buf(buf);

        Ok(())
    }

    pub fn read(&mut self, stream_id: u32, buf: &mut [u8]) -> Result<usize> {
        self.extend_streams(stream_id);
        let ref mut stream = self.streams[stream_id as usize];
        stream.read(buf)
    }

    pub fn finalize_outgoing_stream(&mut self, stream_id: u32) -> Result<()> {
        self.extend_streams(stream_id);
        let ref mut stream = self.streams[stream_id as usize];
        stream.finalize_outgoing();

        Ok(())
    }

    pub fn any_data_available(&self) -> bool {
        self.streams.iter().any(|stream| stream.data_available())
    }

    pub fn data_available(&self, stream_id: u32) -> bool {
        match self.streams.get(stream_id as usize) {
            Some(stream) => stream.data_available(),
            None => false,
        }
    }

    pub fn drain_outgoing_packets(&mut self) -> Vec<packets::Packet> {
        let mut packets = vec![];
        let mut frames = vec![];
        let mut data_length = 0;

        for stream in &mut self.streams {
            let (mut next_outgoing_offset, stream_buffer) = stream.drain_outgoing_buffer();
            let mut stream_buffer = &stream_buffer[..];

            while !stream_buffer.is_empty() {
                let can_fit = MAX_DATA_SIZE - data_length;
                let will_fit = min(can_fit, stream_buffer.len());
                if will_fit > 0 {
                    frames.push(Frame::Stream(
                        stream::StreamFrame {
                            stream_id: stream.id,
                            offset: next_outgoing_offset,
                            stream_data: Vec::from(&stream_buffer[..will_fit]),
                            fin: false,
                        }
                    ));

                    stream_buffer = &stream_buffer[will_fit..];
                    next_outgoing_offset += will_fit as u64;
                }

                if !stream_buffer.is_empty() {
                    packets.push(Self::create_stream_packet(self.id, frames));
                    frames = vec![];
                    data_length = 0;
                }
            }

            match stream.state {
                StreamState::LocalClosed | StreamState::Closed => {
                    if !stream.fin_sent {
                        stream.fin_sent = true;

                        let frames = vec![
                            Frame::Stream(
                                stream::StreamFrame {
                                    stream_id: stream.id,
                                    offset: stream.outgoing_fin_offset(),
                                    stream_data: vec![],
                                    fin: true,
                                }
                            )
                        ];
                        packets.push(Self::create_stream_packet(self.id, frames));
                    }
                },
                _ => {},
            }
        }

        if !frames.is_empty() {
            packets.push(Self::create_stream_packet(self.id, frames));
        }

        debug!("drain_outgoing_packets len: {}", packets.len());
        packets
    }

    pub fn handle_regular_packet(&mut self, packet: &packets::RegularPacket) {
        for frame in &packet.payload.frames {
            match *frame {
                Frame::Ack(..) => unimplemented!(),
                Frame::Blocked(..) => unimplemented!(),
                Frame::ConnectionClose(..) => unimplemented!(),
                Frame::GoAway(..) => unimplemented!(),
                Frame::Padding(..) => {},
                Frame::Ping(..) => {},
                Frame::RstStream(..) => unimplemented!(),
                Frame::StopWaiting(..) => unimplemented!(),
                Frame::Stream(ref stream_frame) => self.handle_stream_frame(stream_frame),
                Frame::WindowUpdate(..) => unimplemented!(),
            }
        }
    }

    fn handle_stream_frame(&mut self, stream_frame: &stream::StreamFrame) {
        let stream_id = stream_frame.stream_id;
        self.extend_streams(stream_id);

        debug!("Stream frame, data len: {}, fin: {}", stream_frame.stream_data.len(), stream_frame.fin);

        let ref mut stream = self.streams[stream_id as usize];
        match stream.extend_incoming_buf(stream_frame.offset, &stream_frame.stream_data[..]) {
            Ok(()) => {},
            Err(ref e) => {
                debug!("Error: {:?}, dropping frame...", e);
            }
        }

        if stream_frame.fin {
            stream.finalize_incoming(stream_frame.offset);
        }
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

            version: None,
            packet_number: 0,
            payload: packets::PacketPayload {
                frames: frames,
            },
        })
    }

    fn extend_streams(&mut self, stream_id: u32) {
        let next_max_id = self.streams.len() as u32;
        if stream_id < next_max_id {
            return;
        }

        for stream_id in next_max_id..(stream_id + 1) {
            self.streams.push(Stream::new(stream_id));
        }
    }
}
