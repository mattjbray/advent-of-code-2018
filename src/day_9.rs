use std::collections::HashMap;

pub fn run() {
    let part_1_solution = ::day_4::max_entry(&play(455, 71223));
    println!("Day 9, part 1: {:?}", part_1_solution);

    // let part_2_solution = ::day_4::max_entry(&play(455, 71223 * 100));
    // println!("Day 9, part 2: {:?}", part_2_solution);
}

type Player = u32;
type Score = u32;

fn play(players: u32, last_marble: u32) -> HashMap<Player, Score> {
    let mut player = 0;
    let mut circle = vec![0];
    let mut current_marble_idx = 0;
    let mut player_scores = HashMap::new();

    for marble in 1..last_marble + 1 {
        player = (player + 1) % players;
        if marble % 23 == 0 {
            current_marble_idx =
                (current_marble_idx + circle.len() - 7) % circle.len();
            let removed_marble = circle.remove(current_marble_idx);
            let added_score = removed_marble + marble;
            player_scores
                .entry(player)
                .and_modify(|score| *score += added_score)
                .or_insert(added_score);
        } else {
            current_marble_idx = (current_marble_idx + 2) % circle.len();
            circle.insert(current_marble_idx, marble);
        }
    }

    player_scores
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play() {
        assert_eq!(::day_4::max_entry(&play(9, 25)), Some((5, 32)));
        assert_eq!(::day_4::max_entry(&play(10, 1618)).map(|t| t.1), Some(8317));
        assert_eq!(::day_4::max_entry(&play(13, 7999)).map(|t| t.1), Some(146373));
        assert_eq!(::day_4::max_entry(&play(17, 1104)).map(|t| t.1), Some(2764));
        assert_eq!(::day_4::max_entry(&play(21, 6111)).map(|t| t.1), Some(54718));
        assert_eq!(::day_4::max_entry(&play(30, 5807)).map(|t| t.1), Some(37305));
    }
}
