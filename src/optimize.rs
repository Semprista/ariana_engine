use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ArianaError {
    NoFreeTimeSlots,
    SchedulableEventIncoherence,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum WeekDay {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventCategory {
    pub index: u8,
    pub label: String,
    pub timeslots: HashMap<WeekDay, Timeslot>,
}

impl EventCategory {
    pub fn new(index: u8, label: &str, timeslots: HashMap<WeekDay, Timeslot>) -> Self {
        Self {
            index,
            label: label.to_string(),
            timeslots,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchedulableEvent {
    pub label: String,
    pub categories_indexes: Vec<u8>, // This allows us to combine two preferences (such as DEFAULT and SPORT)
    pub weekly_duration_ms: u64,
    pub min_session_duration: u64,
    pub max_session_duration: u64,
    pub remaining_duration_ms: u64,
}

impl SchedulableEvent {
    pub fn new(
        label: &str,
        categories_indexes: Option<Vec<u8>>,
        weekly_duration_ms: u64,
        min_session_duration: u64,
        max_session_duration: u64,
    ) -> Self {
        Self {
            label: label.to_string(),
            categories_indexes: categories_indexes.unwrap_or(vec![]),
            weekly_duration_ms,
            min_session_duration,
            max_session_duration,
            remaining_duration_ms: weekly_duration_ms,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub label: String,
    pub day: WeekDay,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
}

impl Event {
    pub fn new(label: &str, day: WeekDay, start_time_ms: u64, end_time_ms: u64) -> Self {
        Self {
            label: label.to_string(),
            day,
            start_time_ms,
            end_time_ms,
        }
    }

    pub fn duration(&self) -> u64 {
        self.end_time_ms - self.start_time_ms
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Planning {
    pub events: HashMap<WeekDay, Vec<Event>>,
}

impl Planning {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn day_total_duration_ms(&self, day: WeekDay) -> u64 {
        self.events
            .get(&day)
            .map(|events| events.iter().map(|e| e.duration()).sum())
            .unwrap_or(0)
    }

    pub fn events_total_duration_ms(&self) -> u64 {
        self.events
            .values()
            .flatten()
            .map(|event| event.duration())
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeslot {
    pub day: WeekDay,
    pub start_hour_ms: u64,
    pub end_hour_ms: u64,
}

impl Timeslot {
    pub const END_OF_DAY: u64 = 86_399_999;

    pub fn new(day: WeekDay, start_hour_ms: u64, end_hour_ms: u64) -> Self {
        Self {
            day,
            start_hour_ms,
            end_hour_ms,
        }
    }

    pub fn full_day_timeslot(day: WeekDay) -> Self {
        Timeslot {
            day,
            start_hour_ms: 0,        // 00:00
            end_hour_ms: 86_399_999, // 23:59:59:999
        }
    }

    pub fn duration(&self) -> u64 {
        self.end_hour_ms.saturating_sub(self.start_hour_ms)
    }

    pub fn intersect(&self, other: &Timeslot) -> Option<Timeslot> {
        if self.day != other.day {
            return None;
        }

        let start = self.start_hour_ms.max(other.start_hour_ms);
        let end = self.end_hour_ms.min(other.end_hour_ms);

        if start < end {
            Some(Timeslot::new(self.day, start, end))
        } else {
            None
        }
    }
}

pub fn optimize(
    current_planning: Planning,
    events_to_schedule: Vec<SchedulableEvent>,
    categories: Vec<EventCategory>,
) -> Result<Planning, ArianaError> {
    let mut week_free_slots: Vec<Timeslot> = vec![];

    /*
     * Search all free slots on the week
     * We loop on each day because it's easier
     */
    for (week_day, events_on_that_day) in &current_planning.events {
        let mut day_free_slots = find_free_slots(week_day, events_on_that_day);
        week_free_slots.append(&mut day_free_slots);
    }

    if week_free_slots.is_empty() {
        return Err(ArianaError::NoFreeTimeSlots);
    }

    let optimized_planning_events: Planning = schedule_events(
        current_planning,
        events_to_schedule,
        week_free_slots,
        categories,
    )?;

    Ok(optimized_planning_events)
}

pub fn find_free_slots(day: &WeekDay, events: &[Event]) -> Vec<Timeslot> {
    let mut free_slots: Vec<Timeslot> = Vec::new();

    let mut sorted_events: Vec<Event> = events.to_vec();
    sorted_events.sort_by_key(|e| e.start_time_ms);

    if sorted_events.is_empty() {
        free_slots.push(Timeslot::full_day_timeslot(*day));
    } else {
        let mut current_time: u64 = 0;

        for event in sorted_events {
            if event.start_time_ms > current_time {
                free_slots.push(Timeslot {
                    day: *day,
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
                day: *day,
                start_hour_ms: current_time,
                end_hour_ms: Timeslot::END_OF_DAY,
            });
        }
    }

    free_slots
}

pub fn schedule_events(
    mut current_planning: Planning,
    mut events_to_schedule: Vec<SchedulableEvent>,
    mut free_slots: Vec<Timeslot>,
    categories: Vec<EventCategory>,
) -> Result<Planning, ArianaError> {
    let all_week_days: [WeekDay; 7] = [
        WeekDay::MONDAY,
        WeekDay::TUESDAY,
        WeekDay::WEDNESDAY,
        WeekDay::THURSDAY,
        WeekDay::FRIDAY,
        WeekDay::SATURDAY,
        WeekDay::SUNDAY,
    ];

    let categories_map: HashMap<u8, &EventCategory> =
        categories.iter().map(|c| (c.index, c)).collect();

    while let Some(event) = events_to_schedule
        .iter_mut()
        .find(|e| e.remaining_duration_ms > 0)
    {
        // Sort days by total duration in current planning to balance the load
        let mut days_sorted = all_week_days;
        days_sorted.sort_by_key(|day| current_planning.day_total_duration_ms(*day));

        let mut slot_found: Option<(usize, WeekDay, u64, u64)> = None;

        for day in days_sorted {
            // Compute the allowed timeslot by intersecting all specified categories for this day
            let allowed_window = compute_allowed_window_for_event(event, day, &categories_map);

            let Some(allowed) = allowed_window else {
                continue;
            };

            for (idx, slot) in free_slots.iter().enumerate() {
                if slot.day == day {
                    if let Some(intersection) = slot.intersect(&allowed) {
                        if intersection.duration() >= event.min_session_duration {
                            slot_found = Some((
                                idx,
                                day,
                                intersection.start_hour_ms,
                                intersection.duration(),
                            ));
                            break;
                        }
                    }
                }
            }

            if slot_found.is_some() {
                break;
            }
        }

        let Some((slot_idx, day, session_start, available_duration)) = slot_found else {
            break;
        };

        let session_duration = event
            .max_session_duration
            .min(event.remaining_duration_ms)
            .min(available_duration);

        let session_end = session_start + session_duration;

        event.remaining_duration_ms -= session_duration;

        // Split the original free slot around the placed session
        let original_slot = free_slots.remove(slot_idx);
        let mut new_slots = Vec::new();

        if original_slot.start_hour_ms < session_start {
            new_slots.push(Timeslot::new(
                day,
                original_slot.start_hour_ms,
                session_start,
            ));
        }

        if original_slot.end_hour_ms > session_end {
            new_slots.push(Timeslot::new(day, session_end, original_slot.end_hour_ms));
        }

        // Insert new slots back
        for s in new_slots {
            if s.duration() > 0 {
                free_slots.push(s);
            }
        }

        // Add the scheduled event into current_planning
        let scheduled_event = Event::new(&event.label, day, session_start, session_end);
        current_planning
            .events
            .entry(day)
            .or_default()
            .push(scheduled_event);
    }

    Ok(current_planning)
}

fn compute_allowed_window_for_event(
    event: &SchedulableEvent,
    day: WeekDay,
    categories_map: &HashMap<u8, &EventCategory>,
) -> Option<Timeslot> {
    if event.categories_indexes.is_empty() {
        return Some(Timeslot::full_day_timeslot(day));
    }

    let mut current_window = Timeslot::full_day_timeslot(day);

    for cat_idx in &event.categories_indexes {
        if let Some(category) = categories_map.get(cat_idx) {
            let cat_slot = category.timeslots.get(&day)?;
            current_window = current_window.intersect(cat_slot)?;
        }
    }

    Some(current_window)
}

#[cfg(test)]
mod tests {
    use super::{
        Event, EventCategory, Planning, SchedulableEvent, Timeslot,
        WeekDay::{FRIDAY, MONDAY, TUESDAY},
        find_free_slots, optimize, schedule_events,
    };

    fn setup_no_event_free_slot() -> Vec<Timeslot> {
        vec![Timeslot::full_day_timeslot(MONDAY)]
    }

    fn setup_one_fullday_event_schedule() -> Vec<Event> {
        vec![
            Event::new("Focus", MONDAY, 0, Timeslot::END_OF_DAY), // 00:00 - 23:59
        ]
    }

    fn setup_midnight_finishing_event_schedule() -> Vec<Event> {
        vec![
            Event::new(
                "Focus",
                MONDAY,
                21 * 60 * 60 * 1000 + 45 * 60 * 1000,
                Timeslot::END_OF_DAY,
            ), // 21:45 - 23:59
        ]
    }

    fn setup_midnight_finishing_event_free_slots() -> Vec<Timeslot> {
        vec![Timeslot::new(MONDAY, 0, 78_300_000)]
    }

    fn setup_following_events_schedule() -> Vec<Event> {
        vec![
            Event::new(
                "Focus",
                MONDAY,
                21 * 60 * 60 * 1000 + 45 * 60 * 1000,
                22 * 60 * 60 * 1000,
            ), // 21:45 - 22:00
            Event::new("Focus", MONDAY, 22 * 60 * 60 * 1000, 23 * 60 * 60 * 1000), // 22:00 - 23:00
        ]
    }

    fn setup_following_events_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(MONDAY, 0, 78_300_000),
            Timeslot::new(MONDAY, 82_800_000, Timeslot::END_OF_DAY),
        ]
    }

    fn setup_one_event_schedule() -> Vec<Event> {
        vec![
            Event::new("Focus", MONDAY, 11 * 60 * 60 * 1000, 13 * 60 * 60 * 1000), // 11:00 - 13:00
        ]
    }

    fn setup_one_event_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(MONDAY, 0, 39_600_000),
            Timeslot::new(MONDAY, 46_800_000, Timeslot::END_OF_DAY),
        ]
    }

    fn setup_two_events_schedule() -> Vec<Event> {
        vec![
            Event::new(
                "Focus A",
                MONDAY,
                10 * 60 * 60 * 1000 + 15 * 60 * 1000,
                12 * 60 * 60 * 1000,
            ), // 10:15 - 12:00
            Event::new(
                "Focus B",
                MONDAY,
                16 * 60 * 60 * 1000 + 15 * 60 * 1000,
                18 * 60 * 60 * 1000,
            ), // 16:15 - 18:00
        ]
    }

    fn setup_two_events_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(MONDAY, 0, 36_900_000),
            Timeslot::new(MONDAY, 43_200_000, 58_500_000),
            Timeslot::new(MONDAY, 64_800_000, Timeslot::END_OF_DAY),
        ]
    }

    fn setup_conflicting_events_schedule() -> Vec<Event> {
        vec![
            Event::new(
                "Meeting",
                MONDAY,
                11 * 60 * 60 * 1000 + 30 * 60 * 1000,
                13 * 60 * 60 * 1000,
            ), // 11:30 - 13:00
            Event::new("Focus", MONDAY, 11 * 60 * 60 * 1000, 16 * 60 * 60 * 1000), // 11:00 - 16:00
            Event::new(
                "Design",
                MONDAY,
                17 * 60 * 60 * 1000 + 30 * 60 * 1000,
                19 * 60 * 60 * 1000,
            ), // 17:30 - 19:00
        ]
    }

    fn setup_conflicting_events_free_slots() -> Vec<Timeslot> {
        vec![
            Timeslot::new(MONDAY, 0, 39600000),
            Timeslot::new(MONDAY, 57600000, 63000000),
            Timeslot::new(MONDAY, 68400000, 86399999),
        ]
    }

    #[test]
    fn day_with_no_event() {
        let free_slots = find_free_slots(&MONDAY, &vec![]);
        assert_eq!(free_slots, setup_no_event_free_slot());
    }

    #[test]
    fn day_with_one_fullday_event() {
        let schedule = setup_one_fullday_event_schedule();
        let free_slots = find_free_slots(&MONDAY, &schedule);
        assert_eq!(free_slots, vec![]);
    }

    #[test]
    fn day_with_events_following() {
        let schedule = setup_following_events_schedule();
        let free_slots = find_free_slots(&MONDAY, &schedule);
        assert_eq!(free_slots, setup_following_events_free_slots());
    }

    #[test]
    fn day_with_midnight_finishing_event() {
        let schedule = setup_midnight_finishing_event_schedule();
        let free_slots = find_free_slots(&MONDAY, &schedule);
        assert_eq!(free_slots, setup_midnight_finishing_event_free_slots());
    }

    #[test]
    fn day_with_one_event() {
        let schedule = setup_one_event_schedule();
        let free_slots = find_free_slots(&MONDAY, &schedule);
        assert_eq!(free_slots, setup_one_event_free_slots());
    }

    #[test]
    fn day_with_two_events() {
        let schedule = setup_two_events_schedule();
        let free_slots = find_free_slots(&MONDAY, &schedule);
        assert_eq!(free_slots, setup_two_events_free_slots());
    }

    #[test]
    fn day_with_conflicting_events() {
        let schedule = setup_conflicting_events_schedule();
        let free_slots = find_free_slots(&MONDAY, &schedule);
        assert_eq!(free_slots, setup_conflicting_events_free_slots());
    }

    #[test]
    fn schedule_event_test() {
        let free_slots = vec![
            Timeslot::new(MONDAY, 8 * 60 * 60 * 1000, 10 * 60 * 60 * 1000),
            Timeslot::new(MONDAY, 13 * 60 * 60 * 1000, 17 * 60 * 60 * 1000),
            Timeslot::new(TUESDAY, 9 * 60 * 60 * 1000, 12 * 60 * 60 * 1000),
            Timeslot::full_day_timeslot(FRIDAY),
        ];

        let events_to_schedule = vec![];

        let result = schedule_events(Planning::new(), events_to_schedule, free_slots, vec![]);
        assert_eq!(result, Ok(Planning::new()));
    }

    #[test]
    fn optimize_places_events_correctly() {
        let events_to_schedule = vec![SchedulableEvent::new(
            "Sport",
            None,
            2 * 60 * 60 * 1000, // 2h total
            1 * 60 * 60 * 1000, // min 1h
            2 * 60 * 60 * 1000, // max 2h
        )];

        let mut current_planning = Planning::new();
        current_planning.events.insert(
            MONDAY,
            vec![Event::new(
                "Work",
                MONDAY,
                8 * 60 * 60 * 1000,
                18 * 60 * 60 * 1000,
            )],
        );

        let result = optimize(current_planning, events_to_schedule, vec![]);
        assert!(result.is_ok());
        let optimized_planning = result.unwrap();
        let all_events: Vec<&Event> = optimized_planning.events.values().flatten().collect();
        let sport_events: Vec<&&Event> = all_events.iter().filter(|e| e.label == "Sport").collect();
        assert!(!sport_events.is_empty());
        let sport_total_duration: u64 = sport_events.iter().map(|e| e.duration()).sum();
        assert_eq!(sport_total_duration, 2 * 60 * 60 * 1000);
    }

    #[test]
    fn optimize_places_events_within_category_timeslot() {
        use std::collections::HashMap;

        // Category 1: Only in the evening (18:00 - 21:00) on Monday
        let mut monday_cat_slots = HashMap::new();
        monday_cat_slots.insert(
            MONDAY,
            Timeslot::new(MONDAY, 18 * 60 * 60 * 1000, 21 * 60 * 60 * 1000),
        );
        let evening_category = EventCategory::new(1, "Evening", monday_cat_slots);

        let events_to_schedule = vec![SchedulableEvent::new(
            "Sport",
            Some(vec![1]),
            2 * 60 * 60 * 1000, // 2h total
            1 * 60 * 60 * 1000, // min 1h
            2 * 60 * 60 * 1000, // max 2h
        )];

        let mut current_planning = Planning::new();
        current_planning.events.insert(
            MONDAY,
            vec![Event::new(
                "Work",
                MONDAY,
                8 * 60 * 60 * 1000,
                17 * 60 * 60 * 1000,
            )],
        );

        let result = optimize(current_planning, events_to_schedule, vec![evening_category]);
        assert!(result.is_ok());
        let optimized = result.unwrap();
        let monday_events = optimized.events.get(&MONDAY).unwrap();
        let sport = monday_events.iter().find(|e| e.label == "Sport").unwrap();

        // The sport event must start at or after 18:00 and end before or at 21:00
        assert!(sport.start_time_ms >= 18 * 60 * 60 * 1000);
        assert!(sport.end_time_ms <= 21 * 60 * 60 * 1000);
        assert_eq!(sport.duration(), 2 * 60 * 60 * 1000);
    }
}
