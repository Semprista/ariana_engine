pub mod day;
pub mod event;
pub mod optimize;
pub mod timeslot;
pub mod timestamp_ms;
pub mod week_event;

#[cfg(test)]
mod tests {
    use crate::{day::Day::MONDAY, event::Event, week_event::WeekEvent};

    use super::*;

    fn setup_event_to_schedule() -> Vec<Event> {
        vec![
            Event::new("Sport", 2 * 60 * 60 * 1000, 60 * 60 * 1000, 60 * 60 * 1000),
            Event::new(
                "SWT",
                4 * 60 * 60 * 1000,
                3 * 60 * 60 * 1000,
                60 * 60 * 1000,
            ),
        ]
    }

    fn setup_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new(
                "Réunion B",
                MONDAY,
                11 * 60 * 60 * 1000 + 30 * 60 * 1000,
                13 * 60 * 60 * 1000,
            ), // 11:30 - 13:00
            WeekEvent::new(
                "Café",
                MONDAY,
                14 * 60 * 60 * 1000 + 15 * 60 * 1000,
                16 * 60 * 60 * 1000,
            ), // 14:15 - 16:00
            WeekEvent::new(
                "Design",
                MONDAY,
                17 * 60 * 60 * 1000 + 30 * 60 * 1000,
                19 * 60 * 60 * 1000,
            ), // 17:30 - 19:00
        ]
    }

    #[test]
    fn it_works() {
        let events_to_schedule = setup_event_to_schedule();
        let schedule = setup_schedule();

        optimize::optimize(events_to_schedule, schedule);
    }
}
