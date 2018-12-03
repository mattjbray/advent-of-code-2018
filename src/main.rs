
fn main() {
    let freqs = aoc::day_1::parse_input("data/day_1.txt");

    let day_1_solution = aoc::day_1::part_1(&freqs);
    println!("Day 1: {}", day_1_solution);

    let day_1_part_2_solution = aoc::day_1::part_2(&freqs);
    println!("Day 1, part 2: {}", day_1_part_2_solution);
}
