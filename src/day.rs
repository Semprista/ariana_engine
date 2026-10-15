use crate::day::Day::{FRIDAY, MONDAY, SATURDAY, SUNDAY, THURDSAY, TUESDAY, WEDNESDAY};
use std::slice::Iter;

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum Day {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURDSAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

impl Day {
    pub fn iterator() -> Iter<'static, Day> {
        static DAYS: [Day; 7] = [
            MONDAY, TUESDAY, WEDNESDAY, THURDSAY, FRIDAY, SATURDAY, SUNDAY,
        ];
        DAYS.iter()
    }
}
