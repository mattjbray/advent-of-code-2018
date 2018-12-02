use std::collections::HashSet;

fn main() {
    let day_1_input =
        std::fs::read_to_string("data/day_1.txt").expect("Couldn't read data/day_1.txt");
    let day_1_input = day_1_input.lines().collect();

    let day_1_solution = day_1(&day_1_input);
    println!("Day 1: {}", day_1_solution);

    let day_1_part_2_solution = day_1_part_2(&day_1_input);
    println!("Day 1, part 2: {}", day_1_part_2_solution);
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
    assert_eq!(day_1(&vec!["+1", "-2", "+3", "+1"]), 3);
    assert_eq!(day_1(&vec!["+1", "+1", "+1"]), 3);
    assert_eq!(day_1(&vec!["+1", "+1", "-2"]), 0);
    assert_eq!(day_1(&vec!["-1", "-2", "-3"]), -6);
}

fn day_1_part_2(frequencies: &Vec<&str>) -> i32 {
    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    let mut current_freq = 0;
    seen_frequencies.insert(current_freq);
    loop {
        for freq in frequencies {
            let f: i32 = freq.parse().expect("Bad frequency");
            current_freq += f;
            if seen_frequencies.contains(&current_freq) {
                return current_freq;
            }
            seen_frequencies.insert(current_freq);
        }
    }
}

#[test]
fn day_1_part_2_test() {
    assert_eq!(day_1_part_2(&vec!["+1", "-2", "+3", "+1"]), 2);
    assert_eq!(day_1_part_2(&vec!["+1", "-1"]), 0);
    assert_eq!(day_1_part_2(&vec!["+3", "+3", "+4", "-2", "-4"]), 10);
    assert_eq!(day_1_part_2(&vec!["-6", "+3", "+8", "+5", "-6"]), 5);
    assert_eq!(day_1_part_2(&vec!["+7", "+7", "-2", "-7", "-4"]), 14);
}
