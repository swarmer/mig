pub mod connection;
pub mod stream;
pub mod timer;
pub mod udp_packet;

use std::collections::{VecDeque, HashMap};
use std::io;
use std::net;
use std::time;

use rand;
use rand::Rng;

use quic::endpoint_role::EndpointRole;
use quic::errors::{Error, Result};
use quic::packets;
use self::connection::Connection;
use self::udp_packet::{IncomingUdpPacket, OutgoingUdpPacket};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuicEngine<T: timer::Timer> {
    timer: T,

    accept_connections: bool,
    connections: HashMap<u64, Connection>,
    new_connection_ids: VecDeque<u64>,

    pending_packets: Vec<OutgoingUdpPacket>,
}

impl <T: timer::Timer> QuicEngine<T> {
    pub fn new(timer: T, accept_connections: bool) -> QuicEngine<T> {
        QuicEngine {
            timer: timer,

            accept_connections: accept_connections,
            connections: HashMap::new(),
            new_connection_ids: VecDeque::new(),

            pending_packets: Vec::new(),
        }
    }

    pub fn initiate_connection(&mut self, addr: net::SocketAddr) -> u64 {
        let mut rng = rand::thread_rng();
        let connection_id = rng.gen();

        let connection = Connection::new(connection_id, EndpointRole::Client, addr);
        self.connections.insert(connection_id, connection);

        debug!("Initiating connection (id: {})", connection_id);
        connection_id
    }

    fn accept_connection(&mut self, connection_id: u64, addr: net::SocketAddr) {
        let connection = Connection::new(connection_id, EndpointRole::Server, addr);
        self.connections.insert(connection_id, connection);
        self.new_connection_ids.push_back(connection_id);
    }

    pub fn have_connections(&self) -> bool {
        !self.new_connection_ids.is_empty()
    }

    pub fn pop_new_connection(&mut self) -> u64 {
        self.new_connection_ids.pop_front().unwrap()
    }

    pub fn handle_incoming_packet(&mut self, packet: IncomingUdpPacket) {
        let endpoint_role = if self.accept_connections {
            EndpointRole::Server
        } else {
            EndpointRole::Client
        };

        let source_address = packet.source_address;
        let packet = match packets::Packet::decode(&mut io::Cursor::new(packet.payload), endpoint_role) {
            Ok(packet) => packet,
            Err(e) => {
                error!("Error while decoding incoming packet: {}", e);
                return;
            }
        };

        // TODO
        match packet {
            packets::Packet::PublicReset(..) => {
                unimplemented!()
            },
            packets::Packet::Regular(ref regular_packet) => {
                match regular_packet.header.connection_id {
                    Some(connection_id) => {
                        if !self.connections.contains_key(&connection_id) {
                            if self.accept_connections {
                                debug!("Registering connection (id: {})", connection_id);
                                self.accept_connection(connection_id, source_address);
                            } else {
                                warn!("Dropping a packet with unknown connection id, can't accept");
                                return;
                            }
                        }

                        let connection = self.connections.get_mut(&connection_id).unwrap();
                        connection.handle_regular_packet(regular_packet);
                    },
                    None => unimplemented!(),
                }
            },
            packets::Packet::VersionNegotiation(..) => {
                unimplemented!()
            },
        }
    }

    pub fn handle_due_events(&mut self) {
        let due_events = self.timer.pop_due_events();
        // TODO
    }

    pub fn write(&mut self, connection_id: u64, stream_id: u32, buf: &[u8]) -> Result<()> {
        {
            let connection =
                self.connections.get_mut(&connection_id)
                .expect("Invalid connection id");
            connection.write(stream_id, buf)?;
        }

        self.flush_buffered_data();

        Ok(())
    }

    pub fn pop_pending_packets(&mut self) -> Vec<OutgoingUdpPacket> {
        self.pending_packets.drain(..).collect()
    }

    pub fn timer_ref(&self) -> &T {
        &self.timer
    }

    pub fn data_available(&self, connection_id: u64, stream_id: u32) -> bool {
        let connection =
            self.connections.get(&connection_id)
            .expect("Invalid connection id");

        connection.data_available(stream_id)
    }

    pub fn read(&mut self, connection_id: u64, stream_id: u32, buf: &mut [u8]) -> Result<usize> {
        let connection =
            self.connections.get_mut(&connection_id)
            .expect("Invalid connection id");

        connection.read(stream_id, buf)
    }

    fn flush_buffered_data(&mut self) {
        for connection in self.connections.values_mut() {
            let peer_address = connection.peer_address();
            for packet in connection.drain_outgoing_packets() {
                let mut buffer = vec![];
                packet.encode(&mut buffer);

                self.pending_packets.push(OutgoingUdpPacket {
                    destination_address: peer_address,
                    payload: buffer,
                });
            }
        }
    }
}
