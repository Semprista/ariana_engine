use crate::{day_name::DayName, timeslot::Timeslot};

#[derive(Debug)]
pub struct DayTimeslots {
    pub day_name: DayName,
    pub timeslots: Vec<Timeslot>,
}

impl DayTimeslots {
    pub const fn new(day: DayName, timeslots: Vec<Timeslot>) -> Self {
        Self {
            day_name: day,
            timeslots: timeslots,
        }
    }
}
