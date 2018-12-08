pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file.");

    let input_chars: Vec<char> = input.chars().collect();

    let part_1_solution = react(input_chars);
    println!("day 5, part 1: {:?}", part_1_solution);
}

fn units_react(c1: char, c2: char) -> bool {
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2
}

fn react(mut polymer: Vec<char>) -> String {
    if polymer.len() < 2 {
        polymer.into_iter().collect()
    } else {
        for j in 1..polymer.len() {
            let i = j - 1;
            if units_react(polymer[i], polymer[j]) {
                // react!
                polymer.remove(j);
                polymer.remove(i);
                return react(polymer);
            }
        }
        polymer.into_iter().collect()
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_react() {
        assert_eq!(react("aA".to_string().chars().collect()), "".to_string());
        assert_eq!(react("abBA".to_string().chars().collect()), "".to_string());
        assert_eq!(
            react("abAB".to_string().chars().collect()),
            "abAB".to_string()
        );
        assert_eq!(
            react("aabAAB".to_string().chars().collect()),
            "aabAAB".to_string()
        );
        assert_eq!(
            react("dabAcCaCBAcCcaDA".to_string().chars().collect()),
            "dabCBAcaDA".to_string()
        );
    }
}
