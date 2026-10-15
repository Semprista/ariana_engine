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

        let _free_slots = find_free_slots(events_on_that_day);
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
    use std::vec;

    use crate::{
        day::Day::MONDAY, optimize::find_free_slots, timeslot::Timeslot, week_event::WeekEvent,
    };

    fn setup_no_event_free_slot() -> Vec<Timeslot> {
        vec![Timeslot::new(0, Timeslot::END_OF_DAY)]
    }

    fn setup_one_fullday_event_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new("Focus", MONDAY, 0, Timeslot::END_OF_DAY), // 00:00 - 23:59
        ]
    }

    fn setup_midnight_finishing_event_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new(
                "Focus",
                MONDAY,
                21 * 60 * 60 * 1000 + 45 * 60 * 1000,
                Timeslot::END_OF_DAY,
            ), // 21:45 - 23:59
        ]
    }

    fn setup_midnight_finishing_event_free_slots() -> Vec<Timeslot> {
        vec![Timeslot::new(0, 78_300_000)]
    }

    fn setup_following_events_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new(
                "Focus",
                MONDAY,
                21 * 60 * 60 * 1000 + 45 * 60 * 1000,
                22 * 60 * 60 * 1000,
            ), // 21:45 - 22:00
            WeekEvent::new("Focus", MONDAY, 22 * 60 * 60 * 1000, 23 * 60 * 60 * 1000), // 22:00 - 23:00
        ]
    }

    fn setup_following_events_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(0, 78_300_000),
            Timeslot::new(82_800_000, Timeslot::END_OF_DAY),
        ]
    }

    fn setup_one_event_schedule() -> Vec<WeekEvent> {
        vec![
            WeekEvent::new("Focus", MONDAY, 11 * 60 * 60 * 1000, 13 * 60 * 60 * 1000), // 11:00 - 13:00
        ]
    }

    fn setup_one_event_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(0, 39_600_000),
            Timeslot::new(46_800_000, Timeslot::END_OF_DAY),
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

    fn setup_two_events_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(0, 36_900_000),
            Timeslot::new(43_200_000, 58_500_000),
            Timeslot::new(64_800_000, Timeslot::END_OF_DAY),
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

    fn setup_conflicting_events_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(0, 39600000),
            Timeslot::new(57600000, 63000000),
            Timeslot::new(68400000, 86399999),
        ]
    }

    #[test]
    fn day_with_no_event() {
        let free_slots = find_free_slots(vec![]);
        assert_eq!(free_slots, setup_no_event_free_slot());
    }

    #[test]
    fn day_with_one_fullday_event() {
        let schedule = setup_one_fullday_event_schedule();
        let events: Vec<&WeekEvent> = schedule.iter().collect();

        let free_slots = find_free_slots(events);
        assert_eq!(free_slots, vec![]);
    }

    #[test]
    fn day_with_events_following() {
        let schedule = setup_following_events_schedule();
        let events: Vec<&WeekEvent> = schedule.iter().collect();

        let free_slots = find_free_slots(events);
        assert_eq!(free_slots, setup_following_events_free_slots());
    }

    #[test]
    fn day_with_midnight_finishing_event() {
        let schedule = setup_midnight_finishing_event_schedule();
        let events: Vec<&WeekEvent> = schedule.iter().collect();

        let free_slots = find_free_slots(events);
        assert_eq!(free_slots, setup_midnight_finishing_event_free_slots());
    }

    #[test]
    fn day_with_one_event() {
        let schedule = setup_one_event_schedule();
        let events: Vec<&WeekEvent> = schedule.iter().collect();

        let free_slots = find_free_slots(events);
        assert_eq!(free_slots, setup_one_event_free_slots());
    }

    #[test]
    fn day_with_two_events() {
        let schedule = setup_two_events_schedule();
        let events: Vec<&WeekEvent> = schedule.iter().collect();

        let free_slots = find_free_slots(events);
        assert_eq!(free_slots, setup_two_events_free_slots());
    }

    #[test]
    fn day_with_conflicting_events() {
        let schedule = setup_conflicting_events_schedule();
        let events: Vec<&WeekEvent> = schedule.iter().collect();

        let free_slots = find_free_slots(events);
        assert_eq!(free_slots, setup_conflicting_events_free_slots());
    }
}
