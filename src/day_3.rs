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

named!(parse_u32 <&str, u32>,
       map_res!(nom::digit, std::str::FromStr::from_str)
);

named!(claim(&str) -> Claim,
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

#[test]
fn test_claim_parser() {
    assert_eq!((parse_u32("123 ")), Ok((" ", 123)));
    assert_eq!(
        claim("#123 @ 3,2: 5x4 "),
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
