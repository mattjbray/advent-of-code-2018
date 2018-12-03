pub fn run(path: &str) {
    let freqs = parse_input(path);

    let part_1_solution = part_1(&freqs);
    println!("Day 1, part 1: {}", part_1_solution);

    let part_2_solution = part_2(&freqs);
    println!("Day 1, part 2: {}", part_2_solution);
}

pub fn parse_input(path: &str) -> Vec<i32> {
    let data = std::fs::read_to_string(path).expect("Couldn't read data file");

    data.lines()
        .map(|f| f.parse().expect("Bad frequency"))
        .collect()
}

pub fn part_1(frequencies: &Vec<i32>) -> i32 {
    frequencies.iter().sum()
}

#[test]
fn day_1_test() {
    assert_eq!(part_1(&vec![1, -2, 3, 1]), 3);
    assert_eq!(part_1(&vec![1, 1, 1]), 3);
    assert_eq!(part_1(&vec![1, 1, -2]), 0);
    assert_eq!(part_1(&vec![-1, -2, -3]), -6);
}

use std::collections::HashSet;

pub fn part_2(frequencies: &Vec<i32>) -> i32 {
    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    let mut current_freq = 0;
    loop {
        for f in frequencies {
            seen_frequencies.insert(current_freq);
            current_freq += f;
            if seen_frequencies.contains(&current_freq) {
                return current_freq;
            }
        }
    }
}

#[test]
fn day_1_part_2_test() {
    assert_eq!(part_2(&vec![1, -2, 3, 1]), 2);
    assert_eq!(part_2(&vec![1, -1]), 0);
    assert_eq!(part_2(&vec![3, 3, 4, -2, -4]), 10);
    assert_eq!(part_2(&vec![-6, 3, 8, 5, -6]), 5);
    assert_eq!(part_2(&vec![7, 7, -2, -7, -4]), 14);
}
