use crate::{day::Day, event::Event, timeslot::Timeslot, week_event::WeekEvent};

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
            for i in 0..events_on_that_day.len() {
                if i == 0 {
                    // first event
                    free_slots.push(Timeslot::new(0, events_on_that_day[i].start_hour_ms));
                } else {
                    free_slots.push(Timeslot::new(
                        events_on_that_day[i - 1].end_hour_ms,
                        events_on_that_day[i].start_hour_ms,
                    ));

                    if i == events_on_that_day.len() - 1 {
                        // last event, so we need to add a slot after it
                        free_slots.push(Timeslot::new(events_on_that_day[i].end_hour_ms, 86399999));
                    }
                }
            }
        }
    }
}
