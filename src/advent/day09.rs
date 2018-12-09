use advent::AdventSolver;
use failure::Error;
use std::collections::VecDeque;

// My input:
const NUM_PLAYERS: usize = 470;
const LAST_MARBLE_VALUE: usize = 72170;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        println!("Winning score with last marble {}: {}", LAST_MARBLE_VALUE,
                 Self::play_game(NUM_PLAYERS, LAST_MARBLE_VALUE));
        println!("Winning score with last marble {}: {}", LAST_MARBLE_VALUE*100,
                 Self::play_game(NUM_PLAYERS, LAST_MARBLE_VALUE*100));
        Ok(())
    }
}

impl Solver {
    fn play_game(num_players: usize, last_marble_value: usize) -> usize {
        let mut circle: VecDeque<isize> = VecDeque::new();
        let mut scores: Vec<usize> = (0..num_players).map(|_| 0).collect();
        circle.push_back(0);

        // "current_pos" is always the front of the deque. Instead of
        // inserting/removing at arbitrary positions, rotate the list and work
        // at the front.
        let mut current_player: usize = 0;
        let mut current_marble: isize = 1;
        while current_marble <= last_marble_value as isize {
            if current_marble % 23 == 0 {
                for _ in 0..7 {
                    let marble = circle.pop_back().unwrap();
                    circle.push_front(marble);
                }
                scores[current_player] += current_marble as usize;
                scores[current_player] += circle.pop_front().unwrap() as usize;
            } else {
                for _ in 0..2 {
                    let marble = circle.pop_front().unwrap();
                    circle.push_back(marble);
                }
                circle.push_front(current_marble);
            }
            current_player = (current_player + 1) % num_players;
            current_marble += 1;
        }
        *(scores.iter().max().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    fn test_part1_examples() {
        // Day 9 is nice enough to include several examples to test!
        assert_eq!(32, Solver::play_game(9, 25));
        assert_eq!(8317, Solver::play_game(10, 1618));
        assert_eq!(146373, Solver::play_game(13, 7999));
        assert_eq!(2764, Solver::play_game(17, 1104));
        assert_eq!(54718, Solver::play_game(21, 6111));
        assert_eq!(37305, Solver::play_game(30, 5807));
    }
}
