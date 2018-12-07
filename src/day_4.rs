use combine::parser::char::{char, digit, newline, string};
use combine::Parser;
use combine::Stream;
use combine::{between, choice, count_min_max, from_str, many1, sep_by};

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file.");

    let (events, _): (Vec<Event>, _) = sep_by(event(), newline())
        .easy_parse(&input[..])
        .expect("Couldn't parse input events");

    println!("{:?}", events);
}

#[derive(Debug, PartialEq)]
struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug, PartialEq)]
enum EventKind {
    BeginShift(u16),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq)]
struct Event {
    ts: Timestamp,
    kind: EventKind,
}

// [1518-11-01 00:00] Guard #10 begins shift

parser!{
    fn timestamp[I]()(I) -> Timestamp
    where [I: Stream<Item = char>]
    {
        let digits_4 = || from_str(count_min_max::<String, _>(4, 4, digit()));
        let digits_2 = || from_str(count_min_max::<String, _>(2, 2, digit()));

        let date =
            (digits_4(),
             char('-'),
             digits_2(),
             char('-'),
             digits_2(),
             char(' '),
             digits_2(),
             char(':'),
             digits_2(),
            ).map(|t| Timestamp { year : t.0, month: t.2, day: t.4,
                                  hour: t.6, minute: t.8
            });
        date

    }
}

parser!{
    fn event_kind[I]()(I) -> EventKind
    where [I: Stream<Item = char>]
    {
        let begin_shift = || between(string("Guard #"), string(" begins shift"),
                                     from_str(many1::<String,_>(digit()))).map(|id| EventKind::BeginShift(id));
        let falls_asleep = || string("falls asleep").map(|_| EventKind::FallsAsleep);
        let wakes_up = || string("wakes up").map(|_| EventKind::WakesUp);

        choice((begin_shift(), falls_asleep(), wakes_up()))
    }
}

parser!{
    fn event[I]()(I) -> Event
    where [I: Stream<Item = char>]
    {
        (between(char('['), char(']'), timestamp()),
         char(' '),
         event_kind()
        ).map(|t| Event {ts: t.0, kind: t.2})
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
            Ok((EventKind::BeginShift(10), ""))
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
                    kind: EventKind::BeginShift(10)
                },
                ""
            ))
        );
    }
}
