use crate::timestamp_ms::TimestampMs;

#[derive(Debug)]
pub struct Event {
    pub label: String,
    pub duration_ms_per_week: TimestampMs,
    pub min_duration_ms_per_session: TimestampMs,
    pub max_duration_ms_per_session: TimestampMs,
}

impl Event {
    pub fn new(
        label: &str,
        duration_ms_per_week: TimestampMs,
        min_duration_ms_per_session: TimestampMs,
        max_duration_ms_per_session: TimestampMs,
    ) -> Self {
        Self {
            label: label.to_string(),
            duration_ms_per_week: duration_ms_per_week,
            min_duration_ms_per_session: min_duration_ms_per_session,
            max_duration_ms_per_session: max_duration_ms_per_session,
        }
    }
}
