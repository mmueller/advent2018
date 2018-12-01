use advent::AdventSolver;
use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let input_sequence = Self::read_input_sequence()?;
        Self::solve1(0, &input_sequence);
        Self::solve2(0, &input_sequence);
        Ok(())
    }
}

impl Solver {
    fn read_input_sequence() -> Result<Vec<isize>, Error> {
        let file = BufReader::new(File::open("input/day01.txt")?);
        let re = Regex::new(r"^(?P<sign>-|\+)(?P<value>\d+)$")?;
        let mut result : Vec<isize> = Vec::new();
        for line in file.lines() {
            let line = line?;
            let caps = match re.captures(&line) {
                Some(caps) => caps,
                None => return Err(format_err!("Didn't match regex: {}", line))
            };
            let sign = &caps["sign"];
            let value = caps["value"].parse::<isize>()?;
            result.push(
                match sign {
                   "-" => -value,
                   "+" => value,
                   _ => return Err(format_err!("Invalid sign: {}", sign))
                }
            );
        }
        Ok(result)
    }

    fn solve1(initial_frequency: isize, input_sequence: &Vec<isize>) {
        let mut freq = initial_frequency;
        for input in input_sequence {
            freq += input;
        }
        println!("Final Frequency: {}", freq);
    }

    fn solve2(initial_frequency: isize, input_sequence: &Vec<isize>) {
        let mut freq = initial_frequency;
        let mut freqs_seen : HashSet<isize> = HashSet::new();
        loop {
            for input in input_sequence {
                if freqs_seen.contains(&freq) {
                    println!("First frequency seen twice: {}", freq);
                    return;
                }
                freqs_seen.insert(freq);
                freq += input;
            }
        }
    }
}
