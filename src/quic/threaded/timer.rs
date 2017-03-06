use std::time;

use quic::engine::timer::{ScheduledEvent, Timer};


#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThreadedTimer {
    // TODO
}

impl ThreadedTimer {
    pub fn new() -> ThreadedTimer {
        ThreadedTimer {}
    }
}

impl Timer for ThreadedTimer {
    fn now(&self) -> time::Instant {
        unimplemented!()
    }

    fn schedule(&mut self, when: time::Duration, event: ScheduledEvent) {
        unimplemented!()
    }
}
