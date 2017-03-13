pub mod connection;
pub mod timer;
pub mod udp_packet;

use std::collections::HashMap;

use self::udp_packet::{IncomingUdpPacket, OutgoingUdpPacket};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuicEngine<T: timer::Timer> {
    timer: T,

    accept_connections: bool,
    connections: HashMap<u64, connection::Connection>,

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

    pub fn handle_incoming_packet(&mut self, packet: IncomingUdpPacket) {
        // TODO
        unimplemented!()
    }

    pub fn handle_timer_event(&mut self, event: timer::ScheduledEvent) {
        // TODO
        unimplemented!()
    }

    pub fn pop_pending_packets(&mut self) -> Vec<OutgoingUdpPacket> {
        self.pending_packets.drain(..).collect()
    }

    pub fn timer_ref(&self, event: timer::ScheduledEvent) -> &T {
        &self.timer
    }
}
