use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        // Load input
        let mut polymer = String::new();
        File::open("input/day05.txt")?.read_to_string(&mut polymer)?;
        polymer = polymer.trim().to_string();

        // Part 1, collapse the input polymer
        let collapsed = Self::collapse_polymer(&polymer);
        println!("Collapsed length: {}", collapsed.len());

        // Part 2, try collapsing with a unit removed
        let result = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .map(|unit| {
                let polymer = Solver::remove_unit(&polymer, unit);
                let collapsed = Self::collapse_polymer(&polymer);
                (unit, collapsed.len())
            })
            .min_by_key(|&(_unit, len)| len)
            .unwrap();
        println!("Without {}, collapsed length: {}", result.0, result.1);
        Ok(())
    }
}

impl Solver {
    fn collapse_polymer(polymer: &str) -> String {
        let mut stack = Vec::new();
        for c in polymer.chars() {
            stack.push(c);
            while stack.len() > 1 &&
                  Self::should_annihilate(stack[stack.len()-2],
                                          stack[stack.len()-1]) {
                stack.pop();
                stack.pop();
            }
        }
        stack.iter().collect()
    }

    // Return the polymer minus any occurrences of a given unit.
    fn remove_unit(polymer: &str, unit: char) -> String {
        polymer.chars()
               .filter(|c| !c.eq_ignore_ascii_case(&unit))
               .collect()
    }

    fn should_annihilate(c1: char, c2: char) -> bool {
        if c1.to_ascii_lowercase() != c2.to_ascii_lowercase() {
            return false;
        }
        c1.is_uppercase() ^ c2.is_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    static EXAMPLE_POLYMER: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn collapse_polymer_example() {
        assert_eq!("dabCBAcaDA", Solver::collapse_polymer(EXAMPLE_POLYMER));
    }

    #[test]
    fn collapse_polymer_empty() {
        assert_eq!("", Solver::collapse_polymer(""));
    }

    #[test]
    fn collapse_polymer_beginning() {
        assert_eq!("bcdefg", Solver::collapse_polymer("aAbcdefg"));
    }

    #[test]
    fn collapse_polymer_ending() {
        assert_eq!("ABCDEF", Solver::collapse_polymer("ABCDEFGg"));
    }

    #[test]
    fn collapse_polymer_into_nothing() {
        assert_eq!("", Solver::collapse_polymer("aBbcDeEdCA"));
    }

    #[test]
    fn collapse_polymer_retains_last_character() {
        assert_eq!("C", Solver::collapse_polymer("aAbBC"));
    }

    #[test]
    fn collapse_polymer_nothing_to_do() {
        assert_eq!("AbCdEfG", Solver::collapse_polymer("AbCdEfG"));
    }
}
