use crate::{day::Day, timestamp_ms::TimestampMs};

#[derive(Debug)]
pub struct WeekEvent {
    pub label: String,
    pub day: Day,
    pub start_hour_ms: TimestampMs,
    pub end_hour_ms: TimestampMs,
}

impl WeekEvent {
    pub fn new(
        label: &str,
        day: Day,
        start_hour_ms: TimestampMs,
        end_hour_ms: TimestampMs,
    ) -> Self {
        Self {
            label: label.to_string(),
            day: day,
            start_hour_ms: start_hour_ms,
            end_hour_ms: end_hour_ms,
        }
    }

    pub fn duration(&self) -> TimestampMs {
        self.end_hour_ms - self.start_hour_ms
    }

    pub fn in_conflict_with(&self, comparing_event: &WeekEvent) -> bool {
        self.start_hour_ms < comparing_event.end_hour_ms
            && comparing_event.start_hour_ms < self.end_hour_ms
    }
}
