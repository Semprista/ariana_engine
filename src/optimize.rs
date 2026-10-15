use crate::{
    day::Day, event::Event, timeslot::Timeslot, timestamp_ms::TimestampMs, week_event::WeekEvent,
};

pub fn optimize(_events_to_schedule: Vec<Event>, typical_week: Vec<WeekEvent>) {
    /*
     * Search all free slots on the week
     * We loop on each day because it's easier
     */
    for day in Day::iterator() {
        let mut free_slots: Vec<Timeslot> = Vec::new();
        // Get all events on that day
        let mut events_on_that_day: Vec<&WeekEvent> = typical_week
            .iter()
            .filter(|event| event.day == *day)
            .collect();

        // Get all theses sorted
        events_on_that_day.sort_by(|a, b| a.start_hour_ms.cmp(&b.start_hour_ms));

        // check if ther is an event today
        if events_on_that_day.is_empty() {
            free_slots.push(Timeslot::FULL_DAY);
        } else {
            let mut current_time: TimestampMs = 0;

            for event in &events_on_that_day {
                if event.start_hour_ms > current_time {
                    free_slots.push(Timeslot::new(current_time, event.start_hour_ms));
                }

                if event.end_hour_ms > current_time {
                    current_time = event.end_hour_ms;
                }
            }

            if current_time < 86399999 {
                free_slots.push(Timeslot::new(current_time, 86399999));
            }
        }

        println!("{:#?}: {:#?}\n{:#?}", day, events_on_that_day, free_slots);
    }
}
