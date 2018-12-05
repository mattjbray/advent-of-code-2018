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

#[derive(Debug, PartialEq)]
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
    let expected_claims = vec![
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
    ];
    assert_eq!(parse_claims(&claims_str), Ok(("", expected_claims)));
}
