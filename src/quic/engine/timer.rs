use std::time;

use quic::packets;


#[derive(Clone, Debug, PartialEq)]
pub enum ScheduledEvent {
    ResendUnackedPacket(packets::Packet),
}

pub trait Timer {
    fn now(&self) -> time::Instant;

    fn schedule(&mut self, when: time::Duration, event: ScheduledEvent);

    fn pop_due_events(&mut self) -> Vec<ScheduledEvent>;
}
