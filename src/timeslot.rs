use crate::timestamp_ms::TimestampMs;

#[derive(Debug, PartialEq)]
pub struct Timeslot {
    pub start_hour_ms: TimestampMs,
    pub end_hour_ms: TimestampMs,
}

impl Timeslot {
    pub const FULL_DAY: Timeslot = Timeslot::new(0, 86399999);

    pub const END_OF_DAY: TimestampMs = 86_399_999;

    pub const fn new(start_hour_ms: TimestampMs, end_hour_ms: TimestampMs) -> Self {
        Self {
            start_hour_ms: start_hour_ms,
            end_hour_ms: end_hour_ms,
        }
    }

    pub const fn duration(&self) -> TimestampMs {
        return self.end_hour_ms - self.start_hour_ms;
    }
}
