// use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;

use combine::{
    easy::Stream,
    parser::char::{newline, spaces, string},
    Parser,
};

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read input file");

    let mut map = Map::parser()
        .easy_parse(&input)
        .expect("Couldn't parse sample points")
        .0;

    println!("Initially:");
    map.draw();

    for i in 0.. {
        map.step();
        println!("After {} steps:", i + 1);
        map.draw();
        std::io::stdin().read_line(&mut String::new());
    }
}

#[derive(Debug)]
struct Map(Vec<Point>);
impl Map {
    fn parser<'a>() -> impl Parser<Input = Stream<&'a str>, Output = Map> {
        combine::sep_end_by::<Vec<_>, _, _>(Point::parser(), newline()).map(|points| Map(points))
    }

    fn step(&mut self) {
        for point in self.0.iter_mut() {
            point.pos.x += point.vel.vx;
            point.pos.y += point.vel.vy;
        }
    }

    fn draw(&self) {
        let map: HashSet<&Position> = self.0.iter().map(|point| (&point.pos)).collect();

        std::io::stdout().write(b"==========\n");
        for y in 0..20 {
            for x in 0..20 {
                if map.contains(&Position::new(x, y)) {
                    std::io::stdout().write(b"#");
                } else {
                    std::io::stdout().write(b".");
                }
            }
            std::io::stdout().write(b"\n");
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    pos: Position,
    vel: Velocity,
}

impl Point {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Point {
        Point {
            pos: Position { x, y },
            vel: Velocity { vx, vy },
        }
    }

    fn parser<'a>() -> impl Parser<Input = Stream<&'a str>, Output = Point> {
        struct_parser!{
            Point {
                pos: Position::parser(),
                _: spaces(),
                vel: Velocity::parser(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn parser<'a>() -> impl Parser<Input = Stream<&'a str>, Output = Position> {
        struct_parser!{
            Position {
                _: string("position=<").skip(spaces()),
                x: parse_i32(),
                _: string(",").skip(spaces()),
                y: parse_i32(),
                _: string(">"),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Velocity {
    vx: i32,
    vy: i32,
}
impl Velocity {
    fn new(vx: i32, vy: i32) -> Velocity {
        Velocity { vx, vy }
    }

    fn parser<'a>() -> impl Parser<Input = Stream<&'a str>, Output = Velocity> {
        struct_parser!{
            Velocity {
                _: string("velocity=<").skip(spaces()),
                vx: parse_i32(),
                _: string(",").skip(spaces()),
                vy: parse_i32(),
                _: string(">"),
            }
        }
    }
}

fn parse_u32<'a>() -> impl Parser<Input = Stream<&'a str>, Output = u32> {
    combine::from_str(combine::parser::range::take_while1(|c: char| {
        c.is_digit(10)
    }))
}

fn parse_i32<'a>() -> impl Parser<Input = Stream<&'a str>, Output = i32> {
    (
        combine::parser::choice::optional(combine::parser::char::char('-')),
        parse_u32(),
    )
        .map(|t| t.1 as i32 * (if t.0.is_some() { -1 } else { 1 }))
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_points() -> &'static str {
        "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"
    }

    #[test]
    fn test_position_parser() {
        assert_eq!(
            Position::parser().easy_parse("position=< -10, 1>"),
            Ok((Position::new(-10, 1), ""))
        );
    }

    #[test]
    fn test_velocity_parser() {
        assert_eq!(
            Velocity::parser().easy_parse("velocity=<1,-10>"),
            Ok((Velocity::new(1, -10), ""))
        );
    }
}
