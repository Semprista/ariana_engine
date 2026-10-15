use crate::{day::Day, timeslot::Timeslot};

pub struct DayTimeslots {
    day: Day,
    timeslots: Vec<Timeslot>,
}

impl DayTimeslots {
    pub const fn new(day: Day, timeslots: Vec<Timeslot>) -> Self {
        Self {
            day: day,
            timeslots: timeslots,
        }
    }
}
