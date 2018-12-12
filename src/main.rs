
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("USAGE: aoc [day] [data_file]")
    }

    let day = &args[1];

    match day.as_ref() {
        "1" => aoc::day_1::run(&args[2]),
        "2" => aoc::day_2::run(&args[2]),
        "3" => aoc::day_3::run(&args[2]),
        "4" => aoc::day_4::run(&args[2]),
        "5" => aoc::day_5::run(&args[2]),
        "6" => aoc::day_6::run(&args[2]),
        "7" => aoc::day_7::run(&args[2]),
        "8" => aoc::day_8::run(&args[2]),
        "9" => aoc::day_9::run(),
        _ => println!("I haven't solved that yet :("),
    }
}
