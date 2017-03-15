pub mod connection;
pub mod timer;
pub mod udp_packet;

use std::collections::HashMap;
use std::time;

use self::udp_packet::{IncomingUdpPacket, OutgoingUdpPacket};

const SEND_DELAY_MS: u64 = 50;


#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuicEngine<T: timer::Timer> {
    timer: T,

    accept_connections: bool,
    connections: HashMap<u64, connection::Connection>,

    pending_packets: Vec<OutgoingUdpPacket>,
}

impl <T: timer::Timer> QuicEngine<T> {
    pub fn new(timer: T, accept_connections: bool) -> QuicEngine<T> {
        let mut engine = QuicEngine {
            timer: timer,

            accept_connections: accept_connections,
            connections: HashMap::new(),

            pending_packets: Vec::new(),
        };

        engine.schedule_send_pending_event();

        engine
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

    fn schedule_send_pending_event(&mut self) {
        self.timer.schedule(
            time::Duration::from_millis(SEND_DELAY_MS),
            timer::ScheduledEvent::SendPendingData,
        );
    }
}
