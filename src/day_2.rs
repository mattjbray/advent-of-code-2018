pub fn run(path: &str) {
    let input = std::fs::read_to_string(path).expect("Couldn't read data file.");
    let boxes: Vec<&str> = input.lines().collect();

    let csum = part_1::run(&boxes[..]);
    println!("Day 2, part 1: {}", csum);
}

mod part_1 {
    pub fn run(boxes: &[&str]) -> u32 {
        checksum(&box_counts(boxes))
    }

    use std::collections::HashMap;

    fn count_chars(input: &str) -> HashMap<char, u32> {
        let mut counts = HashMap::new();

        for c in input.chars() {
            counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        counts
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("aa"), vec![('a', 2)].iter().cloned().collect())
    }

    #[derive(Debug, PartialEq)]
    struct Counts {
        twos: u32,
        threes: u32,
    }

    fn box_counts(boxes: &[&str]) -> Counts {
        let mut counts = Counts { twos: 0, threes: 0 };
        for b in boxes {
            let char_counts = count_chars(b);
            if char_counts.iter().any(|(_, count)| *count == 2) {
                counts.twos += 1;
            }
            if char_counts.iter().any(|(_, count)| *count == 3) {
                counts.threes += 1;
            }
        }
        counts
    }

    fn checksum(counts: &Counts) -> u32 {
        counts.twos * counts.threes
    }

    #[test]
    fn test_part_1() {
        let examples = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(box_counts(&examples[0..1]), Counts { twos: 0, threes: 0 });
        assert_eq!(box_counts(&examples[1..2]), Counts { twos: 1, threes: 1 });
        assert_eq!(box_counts(&examples[2..3]), Counts { twos: 1, threes: 0 });
        assert_eq!(box_counts(&examples[3..4]), Counts { twos: 0, threes: 1 });
        assert_eq!(box_counts(&examples[4..5]), Counts { twos: 1, threes: 0 });
        assert_eq!(box_counts(&examples[5..6]), Counts { twos: 1, threes: 0 });
        assert_eq!(box_counts(&examples[6..7]), Counts { twos: 0, threes: 1 });

        assert_eq!(box_counts(&examples), Counts { twos: 4, threes: 3 });
        assert_eq!(checksum(&box_counts(&examples)), 12);
    }

}
