use crate::{day_name::DayName, timestamp_ms::TimestampMs, week_event::WeekEvent};

pub struct Day {
    pub name: DayName,
    pub events: Vec<WeekEvent>,
}

impl Day {
    pub fn new(name: DayName, events: Vec<WeekEvent>) -> Self {
        Self { name, events }
    }

    pub fn total(&self) -> TimestampMs {
        let mut sum: TimestampMs = 0;
        for event in self.events.iter() {
            sum += event.duration()
        }

        return sum;
    }
}
