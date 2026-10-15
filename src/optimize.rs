use crate::{
    day::Day, event::Event, timeslot::Timeslot, timestamp_ms::TimestampMs, week_event::WeekEvent,
};

pub fn optimize(_events_to_schedule: Vec<Event>, typical_week: Vec<WeekEvent>) {
    /*
     * Search all free slots on the week
     * We loop on each day because it's easier
     */
    for day in Day::iterator() {
        let events_on_that_day: Vec<&WeekEvent> = typical_week
            .iter()
            .filter(|event| event.day == *day)
            .collect();

        let free_slots = find_free_slots(events_on_that_day);
    }
}

fn find_free_slots(mut events: Vec<&WeekEvent>) -> Vec<Timeslot> {
    let mut free_slots: Vec<Timeslot> = Vec::new();

    // Get all events sorted
    events.sort_by(|a, b| a.start_hour_ms.cmp(&b.start_hour_ms));

    // check if ther is an event today
    if events.is_empty() {
        free_slots.push(Timeslot::FULL_DAY);
    } else {
        let mut current_time: TimestampMs = 0;

        for event in &events {
            if event.start_hour_ms > current_time {
                free_slots.push(Timeslot::new(current_time, event.start_hour_ms));
            }

            if event.end_hour_ms > current_time {
                current_time = event.end_hour_ms;
            }
        }

        if current_time < Timeslot::END_OF_DAY {
            free_slots.push(Timeslot::new(current_time, Timeslot::END_OF_DAY));
        }
    }

    return free_slots;
}

#[cfg(test)]
mod tests {
    use crate::{day::Day::MONDAY, optimize::optimize, week_event::WeekEvent};

    fn setup_one_event_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new("Focus", MONDAY, 11 * 60 * 60 * 1000, 13 * 60 * 60 * 1000), // 11:00 - 13:00
        ]
    }

    fn setup_two_events_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new(
                "Focus A",
                MONDAY,
                10 * 60 * 60 * 1000 + 15 * 60 * 1000,
                12 * 60 * 60 * 1000,
            ), // 10:15 - 12:00
            WeekEvent::new(
                "Focus B",
                MONDAY,
                16 * 60 * 60 * 1000 + 15 * 60 * 1000,
                18 * 60 * 60 * 1000,
            ), // 16:15 - 18:00
        ]
    }

    fn setup_conflicting_events_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new(
                "Meeting",
                MONDAY,
                11 * 60 * 60 * 1000 + 30 * 60 * 1000,
                13 * 60 * 60 * 1000,
            ), // 11:30 - 13:00
            WeekEvent::new("Focus", MONDAY, 11 * 60 * 60 * 1000, 16 * 60 * 60 * 1000), // 11:00 - 16:00
            WeekEvent::new(
                "Design",
                MONDAY,
                17 * 60 * 60 * 1000 + 30 * 60 * 1000,
                19 * 60 * 60 * 1000,
            ), // 17:30 - 19:00
        ]
    }

    #[test]
    fn day_with_no_event() {
        optimize(vec![], vec![]);
    }

    #[test]
    fn day_with_one_event() {
        let schedule = setup_one_event_schedule();

        optimize(vec![], schedule);
    }

    #[test]
    fn day_with_two_events() {
        let schedule = setup_two_events_schedule();

        optimize(vec![], schedule);
    }

    #[test]
    fn day_with_conflicting_events() {
        let schedule = setup_conflicting_events_schedule();

        optimize(vec![], schedule);
    }
}
