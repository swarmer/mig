use std::cmp::min;
use std::net;

use quic::endpoint_role::EndpointRole;
use quic::errors::Result;
use quic::packets::frames::{ack, Frame, stream, window_update};
use quic::packets;
use super::stream::{Stream, StreamState};


const MAX_DATA_SIZE: usize = 1000;

#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    id: u64,
    endpoint_role: EndpointRole,
    last_consecutive_packet_number: u64,
    next_outgoing_packet_number: u64,
    peer_address: net::SocketAddr,
    pending_packets: Vec<packets::Packet>,
    streams: Vec<Stream>,

    incoming_packet_count: u64,
    outgoing_packet_count: u64,
}

impl Connection {
    pub fn new(id: u64, endpoint_role: EndpointRole, peer_address: net::SocketAddr) -> Connection {
        Connection {
            id: id,
            endpoint_role: endpoint_role,
            last_consecutive_packet_number: 0,
            next_outgoing_packet_number: 1,
            peer_address: peer_address,
            pending_packets: vec![],
            streams: vec![],

            incoming_packet_count: 0,
            outgoing_packet_count: 0,
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

        packets.extend(self.drain_outgoing_stream_packets());
        packets.extend(self.drain_outgoing_window_update_packets());
        packets.extend(self.drain_pending_packets());

        self.outgoing_packet_count += packets.len() as u64;
        debug!("drain_outgoing_packets len: {}", packets.len());
        if !packets.is_empty() {
            debug!("total outgoing packets: {}", self.outgoing_packet_count);
        }

        for packet in &packets {
            trace!("Sending packet: {:?}", packet);
        }

        return packets;
    }

    pub fn drain_pending_packets(&mut self) -> Vec<packets::Packet> {
        self.pending_packets.drain(..).collect()
    }

    pub fn drain_outgoing_window_update_packets(&mut self) -> Vec<packets::Packet> {
        let mut packets = vec![];

        for stream in &mut self.streams {
            match stream.new_maximum_data() {
                Some(maximum_data) => {
                    packets.push(Self::create_packet(
                        &mut self.next_outgoing_packet_number,
                        self.id,
                        vec![
                            Frame::WindowUpdate(window_update::WindowUpdateFrame {
                                stream_id: stream.id,
                                byte_offset: maximum_data,
                            }),
                        ]
                    ));
                },
                None => {},
            }
        }

        packets
    }

    fn drain_outgoing_stream_packets(&mut self) -> Vec<packets::Packet> {
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
                    packets.push(Self::create_packet(
                        &mut self.next_outgoing_packet_number,self.id, frames));
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
                        packets.push(Self::create_packet(
                            &mut self.next_outgoing_packet_number,self.id, frames));
                    }
                },
                _ => {},
            }
        }

        if !frames.is_empty() {
            packets.push(Self::create_packet(
                &mut self.next_outgoing_packet_number,self.id, frames));
        }

        debug!("drain_outgoing_stream_packets len: {}", packets.len());
        packets
    }

    pub fn handle_regular_packet(&mut self, packet: &packets::RegularPacket) {
        trace!("Received packet: {:?}", packet);

        self.incoming_packet_count += 1;
        debug!("total incoming packets: {}", self.incoming_packet_count);

        for frame in &packet.payload.frames {
            match *frame {
                Frame::Ack(ref ack_frame) =>
                    self.handle_ack_frame(ack_frame),
                Frame::Blocked(..) => unimplemented!(),
                Frame::ConnectionClose(..) => unimplemented!(),
                Frame::GoAway(..) => unimplemented!(),
                Frame::Padding(..) => {},
                Frame::Ping(..) => {},
                Frame::RstStream(..) => unimplemented!(),
                Frame::StopWaiting(..) => unimplemented!(),
                Frame::Stream(ref stream_frame) =>
                    self.handle_stream_frame(stream_frame),
                Frame::WindowUpdate(ref window_update_frame) =>
                    self.handle_window_update_frame(window_update_frame),
            }
        }

        self.save_ack_frame(packet);
    }

    fn handle_window_update_frame(&mut self, wu_frame: &window_update::WindowUpdateFrame) {
        let stream_id = wu_frame.stream_id;
        self.extend_streams(stream_id);

        let stream = &mut self.streams[stream_id as usize];
        stream.max_outgoing_data = wu_frame.byte_offset;
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

    fn handle_ack_frame(&mut self, ack_frame: &ack::AckFrame) {
        // TODO
    }

    fn save_ack_frame(&mut self, packet: &packets::RegularPacket) {
        if packet.packet_number != self.last_consecutive_packet_number + 1 {
            return;
        }

        self.last_consecutive_packet_number = packet.packet_number;

        self.pending_packets.push(Self::create_packet(
            &mut self.next_outgoing_packet_number,self.id, vec![
            Frame::Ack(ack::AckFrame {
                // header
                largest_acknowledged: self.last_consecutive_packet_number,
                ack_delay: 0,

                // ack block section
                first_ack_block_length: 0,
                extra_ack_blocks: vec![],

                // timestamp section
                first_timestamp: None,
                extra_timestamps: vec![],
            }),
        ]));
    }

    pub fn peer_address(&self) -> net::SocketAddr {
        self.peer_address
    }

    fn create_packet(next_packet_number: &mut u64, connection_id: u64, frames: Vec<Frame>) -> packets::Packet {
        let packet_number = *next_packet_number;
        *next_packet_number += 1;

        let header = packets::PacketHeader {
            key_phase: false,
            packet_number_size: 4,
            multipath: false,

            connection_id: Some(connection_id),
        };

        packets::Packet::Regular(packets::RegularPacket {
            header: header,

            version: None,
            packet_number: packet_number,
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
