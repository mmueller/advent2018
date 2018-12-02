use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let ids: Vec<String> =
            BufReader::new(File::open("input/day02.txt")?)
                      .lines()
                      .collect::<Result<Vec<String>, _>>()?;
        println!("Checksum: {}", Self::checksum(&ids));
        match Self::find_similar_ids(&ids) {
            Some(result) => println!("Common characters: {}", result),
            None => println!("Failed to find similar ids!")
        }
        Ok(())
    }
}

impl Solver {
    // Compute the checksum defined in part 1 of the problem.
    fn checksum<T: AsRef<str>>(ids: &[T]) -> usize {
        let count2 = ids.iter()
                        .filter(|&id| Self::has_exactly_n(id.as_ref(), 2))
                        .count();
        let count3 = ids.iter()
                        .filter(|&id| Self::has_exactly_n(id.as_ref(), 3))
                        .count();
        return count2 * count3;
    }

    // Finds the first two ids whose hamming distance is 1, and returns their
    // common characters as a string.
    fn find_similar_ids<T: AsRef<str>>(ids: &[T]) -> Option<String> {
        for id1 in ids {
            for id2 in ids {
                let id1 = id1.as_ref();
                let id2 = id2.as_ref();
                if id1.len() != id2.len() {
                    println!("Bad data, differing lengths: {}, {}", id1, id2);
                } else if Self::hamming_distance(id1, id2) == 1 {
                    println!("Found {} and {}.", id1, id2);
                    return Some(id1.chars().zip(id2.chars())
                                   .filter(|(c1, c2)| c1 == c2)
                                   .map(|(c1, _)| c1)
                                   .collect())
                    
                }
            }
        }
        None
    }

    // Returns true if the given id contains exactly n of any letter.
    // n must be a positive value.
    fn has_exactly_n(id: &str, n: usize) -> bool {
        assert!(n > 0);
        let mut sorted_chars: Vec<char> = id.chars().collect();
        sorted_chars.sort();
        let mut prev_char: char = 0.into();
        let mut counter: usize = 0;
        for c in sorted_chars {
            if c == prev_char {
                counter += 1;
            } else if counter == n {
                return true;
            } else {
                prev_char = c;
                counter = 1;
            }
        }
        return counter == n;
    }

    // Return the number of positions where id1 and id2 have different
    // characters. The strings must have the same length (panics otherwise).
    fn hamming_distance(id1: &str, id2: &str) -> usize {
        assert!(id1.len() == id2.len());
        id1.chars().zip(id2.chars())
           .filter(|(c1, c2)| c1 != c2)
           .count()
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    #[test]
    #[should_panic]
    fn has_exactly_n_panics_with_zero_size() {
        Solver::has_exactly_n("hello world", 0);
    }

    #[test]
    fn has_exactly_n_always_false_with_empty_string() {
        assert!(!Solver::has_exactly_n("", 1));
        assert!(!Solver::has_exactly_n("", 2));
        assert!(!Solver::has_exactly_n("", 3));
    }

    #[test]
    fn has_exactly_n_simple_examples() {
        // One H, two o, three l.
        assert!( Solver::has_exactly_n("Hello, world!", 1));
        assert!( Solver::has_exactly_n("Hello, world!", 2));
        assert!( Solver::has_exactly_n("Hello, world!", 3));
        assert!(!Solver::has_exactly_n("Hello, world!", 4));

        assert!(!Solver::has_exactly_n("aaabbbbb", 1));
        assert!(!Solver::has_exactly_n("aaabbbbb", 2));
        assert!( Solver::has_exactly_n("aaabbbbb", 3));
        assert!(!Solver::has_exactly_n("aaabbbbb", 4));
        assert!( Solver::has_exactly_n("aaabbbbb", 5));
        assert!(!Solver::has_exactly_n("aaabbbbb", 6));
    }

    #[test]
    #[should_panic]
    fn hamming_distance_panics_if_strings_are_not_the_same_length() {
        Solver::hamming_distance("hello", "world!");
    }

    #[test]
    fn hamming_distance_on_empty_strings() {
        assert_eq!(0, Solver::hamming_distance("", ""));
    }

    #[test]
    fn hamming_distance_simple_examples() {
        assert_eq!(0, Solver::hamming_distance("abc", "abc"));
        assert_eq!(4, Solver::hamming_distance("hello", "world"));
        assert_eq!(5, Solver::hamming_distance("abcde", "fghij"));
        assert_eq!(1, Solver::hamming_distance("abcde", "fbcde"));
        assert_eq!(1, Solver::hamming_distance("abcde", "abcdf"));
    }

    lazy_static! {
        static ref EXAMPLE1_IDS: Vec<&'static str> = vec![
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab",
        ];

        static ref EXAMPLE2_IDS: Vec<&'static str> = vec![
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz",
        ];
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(12, Solver::checksum(&EXAMPLE1_IDS));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Some("fgij".to_string()),
                   Solver::find_similar_ids(&EXAMPLE2_IDS));
    }
}
