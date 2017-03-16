pub mod connection;
pub mod timer;
pub mod udp_packet;

use std::collections::HashMap;
use std::time;

use rand;
use rand::Rng;

use quic::endpoint_role::EndpointRole;
use self::connection::Connection;
use self::udp_packet::{IncomingUdpPacket, OutgoingUdpPacket};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuicEngine<T: timer::Timer> {
    timer: T,

    accept_connections: bool,
    connections: HashMap<u64, Connection>,

    pending_packets: Vec<OutgoingUdpPacket>,
}

impl <T: timer::Timer> QuicEngine<T> {
    pub fn new(timer: T, accept_connections: bool) -> QuicEngine<T> {
        QuicEngine {
            timer: timer,

            accept_connections: accept_connections,
            connections: HashMap::new(),

            pending_packets: Vec::new(),
        }
    }

    pub fn initiate_connection(&mut self) -> u64 {
        let mut rng = rand::thread_rng();
        let connection_id = rng.gen();

        let connection = Connection::new(connection_id, EndpointRole::Client);
        self.connections.insert(connection_id, connection);

        connection_id
    }

    pub fn handle_incoming_packet(&mut self, packet: IncomingUdpPacket) {
        // TODO
        unimplemented!()
    }

    pub fn handle_due_events(&mut self) {
        let due_events = self.timer.pop_due_events();
        // TODO
    }

    pub fn pop_pending_packets(&mut self) -> Vec<OutgoingUdpPacket> {
        self.pending_packets.drain(..).collect()
    }

    pub fn timer_ref(&self) -> &T {
        &self.timer
    }
}
