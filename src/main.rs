fn main() {
    let day_1_input =
        std::fs::read_to_string("data/day_1.txt").expect("Couldn't read data/day_1.txt");
    let day_1_solution = day_1(&day_1_input.lines().collect());

    println!("Day 1: {}", day_1_solution);
}

fn day_1(frequencies: &Vec<&str>) -> i32 {
    let mut ans = 0;
    for freq in frequencies {
        let f: i32 = freq.parse().expect("Bad frequency");
        ans += f;
    }
    ans
}

#[test]
fn day_1_test() {
    fn parse(string: &str) -> Vec<&str> {
        string.split(", ").collect()
    }
    let frequencies = parse("+1, -2, +3, +1");
    assert_eq!(day_1(&frequencies), 3);

    let frequencies = parse("+1, +1, +1");
    assert_eq!(day_1(&frequencies), 3);

    let frequencies = parse("+1, +1, -2");
    assert_eq!(day_1(&frequencies), 0);

    let frequencies = parse("-1, -2, -3");
    assert_eq!(day_1(&frequencies), -6);
}
