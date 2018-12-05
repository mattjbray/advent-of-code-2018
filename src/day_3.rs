use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file.");

    let (_, claims) = parse_claims(&input).expect("Couldn't parse claims");
    println!("{:?}", claims)
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    rectangle: Rectangle,
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn claimed_squares(&self) -> HashSet<Square> {
        let mut squares = HashSet::new();
        for x in self.left..(self.left + self.width) {
            for y in self.top..(self.top + self.height) {
                squares.insert(Square { x, y });
            }
        }
        squares
    }
}

fn claimed_square_counts(claims: &[Claim]) -> HashMap<Square, u32> {
    let mut square_counts = HashMap::new();
    for claim in claims {
        for square in claim.rectangle.claimed_squares() {
            square_counts
                .entry(square)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    square_counts
}

fn double_claimed_squares(square_counts: &HashMap<Square, u32>) -> HashSet<&Square> {
    square_counts
        .iter()
        .filter_map(|(square, count)| if *count > 1 { Some(square) } else { None })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Square {
    x: u32,
    y: u32,
}

named!(parse_u32 <&str, u32>,
    map_res!(nom::digit, std::str::FromStr::from_str)
);

named!(parse_claim(&str) -> Claim,
    do_parse!(
        tag!("#") >>
        id: terminated!(parse_u32, tag!(" @ ")) >>
        left: terminated!(parse_u32, tag!(",")) >>
        top: terminated!(parse_u32, tag!(": ")) >>
        width: terminated!(parse_u32, tag!("x")) >>
        height: parse_u32 >>

        ( Claim {
            id,
            rectangle: Rectangle {
                left, top, width, height
            }
        } )
       ));

named!(parse_claims(&str) -> Vec<Claim>,
       terminated!(separated_list_complete!(char!('\n'), parse_claim), tag!("\n"))
);

#[cfg(test)]
mod test {

    use super::*;

    fn example_claims() -> Vec<Claim> {
        vec![
            Claim {
                id: 1,
                rectangle: Rectangle {
                    left: 1,
                    top: 3,
                    width: 4,
                    height: 4,
                },
            },
            Claim {
                id: 2,
                rectangle: Rectangle {
                    left: 3,
                    top: 1,
                    width: 4,
                    height: 4,
                },
            },
            Claim {
                id: 3,
                rectangle: Rectangle {
                    left: 5,
                    top: 5,
                    width: 2,
                    height: 2,
                },
            },
        ]
    }

    #[test]
    fn test_claimed_squares() {
        assert_eq!(
            (Rectangle {
                left: 2,
                top: 3,
                width: 1,
                height: 1,
            }).claimed_squares(),
            vec![Square { x: 2, y: 3 }].into_iter().collect()
        );
    }
    #[test]
    fn test_claimed_square_counts() {
        assert_eq!(
            double_claimed_squares(&claimed_square_counts(&example_claims()[..])),
            [
                Square { x: 4, y: 4 },
                Square { x: 3, y: 4 },
                Square { x: 3, y: 3 },
                Square { x: 4, y: 3 }
            ]
                .iter()
                .collect::<HashSet<&Square>>()
        )
    }

    #[test]
    fn test_claim_parser() {
        assert_eq!((parse_u32("123 ")), Ok((" ", 123)));
        assert_eq!(
            parse_claim("#123 @ 3,2: 5x4 "),
            Ok((
                " ",
                Claim {
                    id: 123,
                    rectangle: Rectangle {
                        left: 3,
                        top: 2,
                        width: 5,
                        height: 4
                    }
                }
            ))
        );
    }

    #[test]
    fn test_claims_parser() {
        let claims_str = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2", ""].join("\n");
        assert_eq!(parse_claims(&claims_str), Ok(("", example_claims())));
    }
}
