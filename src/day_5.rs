use std::collections::HashMap;

pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file.");

    let part_1_solution = react(&input);
    println!("day 5, part 1: {:?}", part_1_solution.len());

    let part_2_solution = part_2(&input);
    println!("day 5, part 2: {:?}", part_2_solution);
}

fn units_react(c1: char, c2: char) -> bool {
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && c1 != c2
}

fn react(polymer: &str) -> String {
    let mut out = String::new();
    let mut last_c = None;

    for c2 in polymer.chars() {
        match last_c {
            None => last_c = Some(c2),
            Some(c1) => {
                if units_react(c1, c2) {
                    // Discard c1 and c2, and go back one char
                    last_c = out.pop();
                } else {
                    out.push(c1);
                    last_c = Some(c2);
                }
            }
        }
    }

    if let Some(c1) = last_c {
        out.push(c1);
    }

    out
}

fn part_2(polymer: &str) -> Option<(char, usize)> {
    let mut map: HashMap<char, usize> = HashMap::new();

    let chars = "abcdefghijklmnopqrstuvwxyz";

    for c in chars.chars() {
        let input = polymer.replace(c, "").replace(c.to_ascii_uppercase(), "");
        let result = react(&input);
        map.insert(c, result.len());
    }

    let ans = map
        .iter()
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .map(|(&c, &len)| (c, len));

    ans
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_react() {
        assert_eq!(react("aA"), "".to_string());
        assert_eq!(react("abBA"), "".to_string());
        assert_eq!(react("abAB"), "abAB".to_string());
        assert_eq!(react("aabAAB"), "aabAAB".to_string());
        assert_eq!(react("dabAcCaCBAcCcaDA"), "dabCBAcaDA".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2("dabAcCaCBAcCcaDA"), Some(('c', 4)));
    }
}
