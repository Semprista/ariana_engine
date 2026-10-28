use crate::day_name::DayName::{FRIDAY, MONDAY, SATURDAY, SUNDAY, THURDSAY, TUESDAY, WEDNESDAY};
use std::slice::Iter;

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum DayName {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURDSAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

impl DayName {
    pub fn iterator() -> Iter<'static, DayName> {
        static DAYS: [DayName; 7] = [
            MONDAY, TUESDAY, WEDNESDAY, THURDSAY, FRIDAY, SATURDAY, SUNDAY,
        ];
        DAYS.iter()
    }
}
