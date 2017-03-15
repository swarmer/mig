use std::time;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScheduledEvent {
    SendPendingData,
}

pub trait Timer {
    fn now(&self) -> time::Instant;

    fn schedule(&mut self, when: time::Duration, event: ScheduledEvent);

    fn pop_due_events(&mut self) -> Vec<ScheduledEvent>;
}
