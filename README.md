# Ariana Engine

Ariana Engine is a Rust library for weekly schedule planning. It provides the basic building blocks for representing planned activities, recurring weekly events, weekdays, and time slots.

The engine can currently identify free time around existing events and detect conflicts between occupied time slots. Its main domain types are:

- `Event`, which describes an activity to schedule;
- `WeekEvent`, which describes an event already occupying part of a typical week;
- `Day`, which represents a weekday;
- `Timeslot`, which represents a time interval in milliseconds from midnight.

## Disclaimer

This project is experimental and still under development. The public `optimize` function currently prepares free slots but does not yet return a complete optimized schedule. The API may change without notice and should not be relied on for production scheduling.

## Development

Run the test suite with:

```bash
cargo test
```

Check compilation with:

```bash
cargo check
```

The current tests cover empty days, consecutive events, overlapping events, events ending at midnight, and full-day events.
