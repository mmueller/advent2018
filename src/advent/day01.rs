use advent::AdventSolver;
use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let input_sequence: Vec<isize> =
            BufReader::new(File::open("input/day01.txt")?)
                      .lines()
                      .collect::<Result<Vec<String>, _>>()?
                      .iter()
                      .map(|s| s.parse::<isize>())
                      .collect::<Result<Vec<isize>, _>>()?;
        Self::solve1(0, &input_sequence);
        Self::solve2(0, &input_sequence);
        Ok(())
    }
}

impl Solver {
    fn solve1(initial_frequency: isize, input_sequence: &Vec<isize>) {
        println!("Final Frequency: {}",
                 initial_frequency + input_sequence.iter().sum::<isize>());
    }

    fn solve2(initial_frequency: isize, input_sequence: &Vec<isize>) {
        let mut freq = initial_frequency;
        let mut freqs_seen = HashSet::new();
        for input in input_sequence.iter().cycle() {
            freq += input;
            if freqs_seen.contains(&freq) {
                break;
            }
            freqs_seen.insert(freq);
        }
        println!("First frequency seen twice: {}", freq);
    }
}
