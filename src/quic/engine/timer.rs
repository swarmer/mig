use std::time;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScheduledEvent {
    // TODO
}

pub trait Timer {
    fn now(&self) -> time::Instant;

    fn schedule(&mut self, when: time::Duration, event: ScheduledEvent);
}
