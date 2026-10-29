use crate::{
    day::Day,
    optimize::WeekDay::{FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY},
    week_event::WeekEvent,
};
use std::vec;

#[derive(Debug)]
enum ArianaError {
    NoFreeTimeSlots,
    SchedulableEventIncoherence,
}

/**
 * TODO: rename to "Day" after day::Day deletion
 */
#[derive(PartialEq, Clone, Copy)]
enum WeekDay {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

struct SchedulableEvent {
    label: String,
    preferred_days: Vec<Day>, // NOT USED AS FOR NOW
    weekly_duration_ms: u64,
    min_session_duration: u64,
    max_session_duration: u64,
}

struct Event {
    label: String,
    day: WeekDay, // TODO: rename
    start_time_ms: u64,
    end_time_ms: u64,
}

struct Planning {
    events: Vec<Event>,
}

struct Timeslot {
    day: WeekDay, // TODO: rename
    start_hour_ms: u64,
    end_hour_ms: u64,
}

impl Timeslot {
    const END_OF_DAY: u64 = 86_399_999;

    fn full_day_timeslot(day: WeekDay) -> Self {
        Timeslot {
            day: day,
            start_hour_ms: 0,        // 00:00
            end_hour_ms: 86_399_999, // 23:59:59:999
        }
    }
}

pub fn optimize(
    current_planning: Planning,
    events_to_schedule: Vec<SchedulableEvent>,
) -> Result<Planning, ArianaError> {
    let week_days: Vec<WeekDay> = vec![
        MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY,
    ];
    let mut week_free_slots: Vec<Timeslot> = vec![];

    /*
     * Search all free slots on the week
     * We loop on each day because it's easier
     */
    for week_day in week_days {
        let events_on_that_day: Vec<&Event> = current_planning
            .events
            .iter()
            .filter(|event| event.day == week_day)
            .collect();

        let mut day_free_slots = find_free_slots(week_day, &events_on_that_day);
        week_free_slots.append(&mut day_free_slots);
    }

    if week_free_slots.is_empty() {
        return Err(ArianaError::NoFreeTimeSlots);
    }

    let optimized_planning_events: Planning =
        schedule_events(current_planning, events_to_schedule, week_free_slots).unwrap();

    return Ok(optimized_planning_events);
}

fn find_free_slots(day: WeekDay, events: &[&Event]) -> Vec<Timeslot> {
    let mut free_slots: Vec<Timeslot> = Vec::new();

    let mut sorted_events: Vec<&Event> = events.to_vec();
    sorted_events.sort_by_key(|e| e.start_time_ms);

    if events.is_empty() {
        free_slots.push(Timeslot::full_day_timeslot(day));
    } else {
        let mut current_time: u64 = 0;

        for event in sorted_events {
            if event.start_time_ms > current_time {
                free_slots.push(Timeslot {
                    day: day,
                    start_hour_ms: current_time,
                    end_hour_ms: event.start_time_ms,
                });
            }

            if event.end_time_ms > current_time {
                current_time = event.end_time_ms;
            }
        }

        if current_time < Timeslot::END_OF_DAY {
            free_slots.push(Timeslot {
                day: day,
                start_hour_ms: current_time,
                end_hour_ms: Timeslot::END_OF_DAY,
            });
        }
    }

    return free_slots;
}

fn schedule_events(
    mut current_planning: Planning,
    mut events_to_schedule: Vec<SchedulableEvent>,
    mut free_slots: Vec<Timeslot>,
) -> Result<Planning, ArianaError> {
    let mut scheduled_events: Vec<WeekEvent> = Vec::new();

    loop {
        let Some(event) = events_to_schedule
            .iter_mut()
            .find(|e| e.remaining_duration_ms > 0)
        else {
            break;
        };

        current_week.sort_by_key(|day| day.total());

        let mut slot_found = None;

        for day in &current_week {
            if let Some(day_slots) = free_slots.iter_mut().find(|ds| ds.day_name == day.name) {
                if let Some((slot_idx, slot)) = day_slots
                    .timeslots
                    .iter()
                    .enumerate()
                    .find(|(_, s)| s.duration() >= event.min_duration_ms_per_session)
                {
                    slot_found = Some((day.name, slot_idx, slot.start_hour_ms, slot.duration()));
                    break;
                }
            }
        }

        let Some((day_name, slot_idx, slot_start, slot_duration)) = slot_found else {
            break;
        };

        let session_duration = event
            .max_duration_ms_per_session
            .min(event.remaining_duration_ms)
            .min(slot_duration);

        let session_start = slot_start;
        let session_end = slot_start + session_duration;

        event.remaining_duration_ms -= session_duration;

        let day_slots = free_slots
            .iter_mut()
            .find(|ds| ds.day_name == day_name)
            .unwrap();
        day_slots.timeslots[slot_idx].start_hour_ms += session_duration;
        if day_slots.timeslots[slot_idx].duration() == 0 {
            day_slots.timeslots.remove(slot_idx);
        }

        if let Some(day) = current_week.iter_mut().find(|d| d.name == day_name) {
            day.events.push(WeekEvent::new(
                &event.label,
                day_name,
                session_start,
                session_end,
            ));
        }

        scheduled_events.push(WeekEvent::new(
            &event.label,
            day_name,
            session_start,
            session_end,
        ));
    }

    scheduled_events
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        day_name::DayName::MONDAY,
        optimize::{find_free_slots, schedule_events},
        timeslot::Timeslot,
        week_event::WeekEvent,
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

    #[test]
    fn schedule_event_test() {
        use crate::{
            day_name::DayName::{FRIDAY, MONDAY, TUESDAY},
            day_timeslots::DayTimeslots,
            timeslot::Timeslot,
        };

        let free_slots = vec![
            DayTimeslots::new(
                MONDAY,
                vec![
                    Timeslot::new(8 * 60 * 60 * 1000, 10 * 60 * 60 * 1000),
                    Timeslot::new(13 * 60 * 60 * 1000, 17 * 60 * 60 * 1000),
                ],
            ),
            DayTimeslots::new(
                TUESDAY,
                vec![Timeslot::new(9 * 60 * 60 * 1000, 12 * 60 * 60 * 1000)],
            ),
            DayTimeslots::new(FRIDAY, vec![Timeslot::new(0, Timeslot::END_OF_DAY)]),
        ];

        let events_to_schedule = vec![];

        let result = schedule_events(vec![], events_to_schedule, free_slots);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn optimize_places_events_correctly() {
        use super::optimize;
        use crate::day_name::DayName;
        use crate::event_to_schedule::EventToSchedule;
        use crate::timestamp_ms::TimestampMs;

        let events_to_schedule = vec![EventToSchedule::new(
            "Sport",
            2 * 60 * 60 * 1000, // 2h total
            1 * 60 * 60 * 1000, // min 1h
            2 * 60 * 60 * 1000, // max 2h
        )];

        let typical_week = vec![WeekEvent::new(
            "Work",
            DayName::MONDAY,
            8 * 60 * 60 * 1000,
            18 * 60 * 60 * 1000,
        )];

        let scheduled = optimize(events_to_schedule, typical_week);
        assert!(!scheduled.is_empty());
        assert_eq!(scheduled[0].label, "Sport");
        let total_duration: TimestampMs = scheduled.iter().map(|e| e.duration()).sum();
        assert_eq!(total_duration, 2 * 60 * 60 * 1000);
    }
}
