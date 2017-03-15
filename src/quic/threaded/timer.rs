use std::time;

use quic::engine::timer::{ScheduledEvent, Timer};


#[derive(Clone, Debug, PartialEq)]
struct ScheduledItem {
    event: ScheduledEvent,
    instant: time::Instant,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ThreadedTimer {
    scheduled_items: Vec<ScheduledItem>,
}

impl ThreadedTimer {
    pub fn new() -> ThreadedTimer {
        ThreadedTimer::default()
    }

    pub fn time_until_next_event(&self) -> Option<time::Duration> {
        self.scheduled_items.iter()
            .map(|item| {
                let now = time::Instant::now();
                if item.instant >= now {
                    item.instant - now
                } else {
                    time::Duration::from_millis(0)
                }
            })
            .min()
    }
}

impl Timer for ThreadedTimer {
    fn now(&self) -> time::Instant {
        time::Instant::now()
    }

    fn schedule(&mut self, when: time::Duration, event: ScheduledEvent) {
        self.scheduled_items.push(ScheduledItem {
            event: event,
            instant: time::Instant::now() + when,
        })
    }

    fn pop_due_events(&mut self) -> Vec<ScheduledEvent> {
        let (pending, due) =
            self.scheduled_items.drain(..)
            .partition(|item| item.instant > time::Instant::now());

        self.scheduled_items = pending;

        due.into_iter().map(|item| item.event).collect()
    }
}
