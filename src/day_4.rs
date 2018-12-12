use combine::parser::char::{char, digit, newline, string};
use combine::Parser;
use combine::Stream;
use combine::{between, choice, count_min_max, from_str, many1, sep_by};

use std::collections::HashMap;

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file.");

    let (mut events, _): (Vec<Event>, _) = sep_by(event(), newline())
        .easy_parse(&input[..])
        .expect("Couldn't parse input events");

    events.sort_by(|e1, e2| e1.ts.cmp(&e2.ts));

    let part_1_solution = part_1(&events);
    println!("day 4, part 1: {:?}", part_1_solution);

    let part_2_solution = part_2(&events);
    println!(
        "day 4, part 2: {:?}",
        part_2_solution.map(|(guard_id, minute)| guard_id * (minute as u16))
    );
}

type Minute = u8;
type GuardID = u16;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: Minute,
}

#[derive(Debug, PartialEq)]
enum EventKind {
    BeginsShift(GuardID),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq)]
struct Event {
    ts: Timestamp,
    kind: EventKind,
}

// Assumes events are sorted
fn minutes_asleep(events: &[Event]) -> HashMap<GuardID, HashMap<Minute, u32>> {
    let mut map: HashMap<GuardID, HashMap<Minute, u32>> = HashMap::new();
    let mut current_guard = None;
    let mut fell_asleep_at = None;
    for event in events {
        match event.kind {
            EventKind::BeginsShift(id) => current_guard = Some(id),
            EventKind::FallsAsleep => fell_asleep_at = Some(event.ts.minute),
            EventKind::WakesUp => {
                match (current_guard, fell_asleep_at) {
                    (Some(guard_id), Some(fell_asleep_at)) => {
                        for minute in fell_asleep_at..event.ts.minute {
                            map.entry(guard_id)
                                .and_modify(|minute_counts| {
                                    minute_counts
                                        .entry(minute)
                                        .and_modify(|count| *count += 1)
                                        .or_insert(1);
                                }).or_insert_with(|| {
                                    let mut m = HashMap::new();
                                    m.insert(minute, 1);
                                    m
                                });
                        }
                    }
                    _ => (),
                }
                fell_asleep_at = None;
            }
        }
    }
    map
}

fn total_minutes_asleep(
    minute_counts_by_guard: &HashMap<GuardID, HashMap<Minute, u32>>,
) -> HashMap<GuardID, u32> {
    let mut map = HashMap::new();
    for (&guard_id, minutes_asleep_count) in minute_counts_by_guard.iter() {
        map.insert(guard_id, minutes_asleep_count.values().sum());
    }
    map
}

pub fn max_entry<K, V>(m: &HashMap<K, V>) -> Option<(K, V)>
where
    K: Eq + std::hash::Hash + Copy,
    V: Ord + Copy,
{
    m.iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .map(|(&k, &v)| (k, v))
}

fn most_sleepy_guard(
    minute_counts_by_guard: &HashMap<GuardID, HashMap<Minute, u32>>,
) -> Option<GuardID> {
    max_entry(&total_minutes_asleep(minute_counts_by_guard)).map(|t| t.0)
}

fn part_1(events: &[Event]) -> Option<u16> {
    let m = minutes_asleep(&events);
    let sleepiest_guard = most_sleepy_guard(&m);
    sleepiest_guard.and_then(|id| {
        m.get(&id)
            .and_then(|minute_counts| max_entry(&minute_counts))
            .map(|(minute, _)| id * (minute as u16))
    })
}

fn part_2(events: &[Event]) -> Option<(GuardID, Minute)> {
    let m = minutes_asleep(events);
    let max_count_by_guard_minute: HashMap<(GuardID, Minute), u32> = m
        .iter()
        .filter_map(|(&guard_id, minute_counts)| {
            let max_minute_count = max_entry(&minute_counts);
            max_minute_count.map(|(minute, count)| ((guard_id, minute), count))
        }).collect();
    max_entry(&max_count_by_guard_minute).map(|t| t.0)
}

// [1518-11-01 00:00] Guard #10 begins shift

parser!{
    fn timestamp[I]()(I) -> Timestamp
    where [I: Stream<Item = char>]
    {
        let digits_u16 = |n| from_str(count_min_max::<String, _>(n, n, digit()));
        let digits_u8 = |n| from_str(count_min_max::<String, _>(n, n, digit()));

        struct_parser!{
            Timestamp {
                year: digits_u16(4),
                _: char('-'),
                month: digits_u8(2),
                _: char('-'),
                day: digits_u8(2),
                _: char(' '),
                hour: digits_u8(2),
                _: char(':'),
                minute: digits_u8(2),
            }
        }

    }
}

parser!{
    fn event_kind[I]()(I) -> EventKind
    where [I: Stream<Item = char>]
    {
        let begin_shift = || between(string("Guard #"), string(" begins shift"),
                                     from_str(many1::<String,_>(digit()))).map(|id| EventKind::BeginsShift(id));
        let falls_asleep = || string("falls asleep").map(|_| EventKind::FallsAsleep);
        let wakes_up = || string("wakes up").map(|_| EventKind::WakesUp);

        choice((begin_shift(), falls_asleep(), wakes_up()))
    }
}

parser!{
    fn event[I]()(I) -> Event
    where [I: Stream<Item = char>]
    {
        struct_parser!{
            Event {
                _: char('['),
                ts: timestamp(),
                _: char(']'),
                _: char(' '),
                kind: event_kind(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use combine::Parser;

    #[test]
    fn test_timestamp_parser() {
        assert_eq!(
            timestamp().easy_parse("2018-08-11 23:58"),
            Ok((
                Timestamp {
                    year: 2018,
                    month: 8,
                    day: 11,
                    hour: 23,
                    minute: 58,
                },
                ""
            ))
        );
    }

    #[test]
    fn test_event_kind_parser() {
        assert_eq!(
            event_kind().easy_parse("Guard #10 begins shift"),
            Ok((EventKind::BeginsShift(10), ""))
        );

        assert_eq!(
            event_kind().easy_parse("falls asleep"),
            Ok((EventKind::FallsAsleep, ""))
        );
        assert_eq!(
            event_kind().easy_parse("wakes up"),
            Ok((EventKind::WakesUp, ""))
        );
    }

    #[test]
    fn test_event_parser() {
        assert_eq!(
            event().easy_parse("[1518-11-01 00:00] Guard #10 begins shift"),
            Ok((
                Event {
                    ts: Timestamp {
                        year: 1518,
                        month: 11,
                        day: 1,
                        hour: 0,
                        minute: 0
                    },
                    kind: EventKind::BeginsShift(10)
                },
                ""
            ))
        );
    }

    #[test]
    fn test_minutes_asleep() {
        let records = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:08] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:07] falls asleep",
            "[1518-11-03 00:08] wakes up",
        ].join("\n");

        let (events, _): (Vec<Event>, _) = sep_by(event(), newline())
            .easy_parse(&records[..])
            .expect("Couldn't parse input events");

        let mut expected = HashMap::new();
        let mut guard_10 = HashMap::new();
        guard_10.insert(5, 1);
        guard_10.insert(6, 1);
        guard_10.insert(7, 2);
        expected.insert(10, guard_10);
        assert_eq!(minutes_asleep(&events), expected);
    }

    fn example_events() -> Vec<Event> {
        let records = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ].join("\n");

        let (events, _): (Vec<Event>, _) = sep_by(event(), newline())
            .easy_parse(&records[..])
            .expect("Couldn't parse input events");
        events
    }

    #[test]
    fn test_part_1() {
        let events = example_events();
        let minutes_asleep_counts = minutes_asleep(&events);

        assert_eq!(
            total_minutes_asleep(&minutes_asleep_counts),
            vec![(10, 50), (99, 30)].into_iter().collect()
        );

        assert_eq!(
            minutes_asleep_counts
                .get(&10)
                .and_then(|minute_counts| max_entry(&minute_counts)),
            Some((24, 2))
        );

        assert_eq!(part_1(&events), Some(240))
    }

    #[test]
    fn test_part_2() {
        let events = example_events();

        assert_eq!(part_2(&events), Some((99, 45)))
    }
}
