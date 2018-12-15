use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::{HashMap,VecDeque};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash,Hasher};
use std::io::{BufRead,BufReader};

#[derive(PartialEq, Eq, Hash)]
pub struct Solver {
    // State contains the continuous range of pots with plants growing in
    // them, plus some padding on left and right.
    state: VecDeque<bool>,
    // This the offset for computing the actual pot number of a given plant in
    // the state vector. state[i] + offset is the "real" pot location.
    offset: i64,
    // The rules we read from the input file.
    rules: Vec<bool>,
    // The current generation, starting at zero.
    generation: u64,
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        self.read_input()?;

        // Map of hash value -> (generation, sum of plant positions).
        let mut seen_states: HashMap<u64, (u64, i64)> = HashMap::new();

        while self.generation < 20 {
            self.spread();
            seen_states.insert(
                self.get_hash(),
                (self.generation, self.sum_of_plant_positions()));
        }
        println!("After 20 generations: {}", self.sum_of_plant_positions());

        // Took a while to discover this, but my input eventually reaches a
        // steady state, except that the pattern is migrating to the right.
        // Since it can only move linearly with respect to generations, we can
        // discover the delta between cycles. My cycle length was 1 so I'm not
        // handling longer cycles, which would be hard. :P
        let delta;
        loop {
            self.spread();
            let hash = self.get_hash();
            let sum = self.sum_of_plant_positions();
            if seen_states.contains_key(&hash) {
                let (prev_gen, prev_sum) = seen_states[&hash];
                println!("Cycle found from generation {} -> {}.",
                         prev_gen, self.generation);
                delta = self.sum_of_plant_positions() - prev_sum;
                break;
            } else {
                seen_states.insert(hash, (self.generation, sum));
            }
        }

        let gens_remaining = 50_000_000_000 - self.generation as i64;
        let result = self.sum_of_plant_positions() + delta * gens_remaining;
        println!("After 50 billion generations: {}", result);
        Ok(())
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver {
            state: VecDeque::new(),
            offset: 0,
            // This assumes the rules are 5 bits long
            rules: vec![false; 32],
            generation: 0,
        }
    }
}

impl Solver {
    fn read_input(&mut self) -> Result<(), Error> {
        let lines = BufReader::new(File::open("input/day12.txt")?)
                              .lines()
                              .collect::<Result<Vec<String>, _>>()?;
        self.parse_initial_state(&lines[0])?;
        self.parse_rules(&lines[2..])?;
        self.pad();
        Ok(())
    }

    fn parse_initial_state(&mut self, line: &str) -> Result<(), Error> {
        let initial_state_re = Regex::new(r"^initial state: ([.#]+)$")?;
        match initial_state_re.captures(line) {
            Some(caps) => {
                for c in caps[1].chars() {
                    self.state.push_back(c == '#');
                }
                Ok(())
            },
            None => {
                Err(format_err!("State parse error: {}", line))
            }
        }
    }

    fn parse_rules(&mut self, lines: &[String]) -> Result<(), Error> {
        let rule_re = Regex::new(r"^([.#]+) => ([.#])$")?;
        for line in lines {
            match rule_re.captures(line) {
                Some(caps) => {
                    let rule_index = caps[1].chars()
                                            .map(|c| (c == '#') as usize)
                                            .fold(0, |acc, x| (acc << 1) + x);
                    self.rules[rule_index] = &caps[2] == "#";
                },
                None => {
                    return Err(format_err!("Rule parse error: {}", line));
                }
            }
        }
        Ok(())
    }

    // Ensures that there are always exactly 5 empty pots at the beginning and
    // end of the state vector.
    fn pad(&mut self) {
        // Calculate how many pots need to be added or removed at each end.
        let front_pad_amount =
            match self.state.iter().position(|&v| v) {
                Some(i) => 5 - i as isize,
                None => 0
            };
        let back_pad_amount =
            match self.state.iter().rposition(|&v| v) {
                Some(i) => 5 - (self.state.len() as isize - 1 - i as isize),
                None => 0
            };
        self.offset -= front_pad_amount as i64;
        // Either push or pop pots from the vector as needed.
        for _ in 0..front_pad_amount {
            self.state.push_front(false);
        }
        for _ in front_pad_amount..0 {
            self.state.pop_front();
        }
        for _ in 0..back_pad_amount {
            self.state.push_back(false);
        }
        for _ in back_pad_amount..0 {
            self.state.pop_back();
        }
    }

    // Run one generation of rule simulation, mutating state appropriately.
    fn spread(&mut self) {
        let state_copy = self.state.clone();
        for i in 2..state_copy.len()-2 {
            self.state[i] = self.apply_rule(&state_copy, i);
        }
        self.pad();
        self.generation += 1;
    }

    fn apply_rule(&self, old_state: &VecDeque<bool>, i: usize) -> bool {
        let rule_index = (i-2..=i+2).map(|i| old_state[i])
                                    .fold(0, |acc, x| (acc<<1) + x as usize);
        self.rules[rule_index]
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.state.hash(&mut hasher);
        hasher.finish()
    }

    fn sum_of_plant_positions(&mut self) -> i64 {
        self.state.iter()
                  .enumerate()
                  .map(|(i, &v)| {
                      if v {
                          i as i64 + self.offset
                      } else {
                          0
                      }
                  })
                  .sum::<i64>()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!("state: {}",
                 self.state.iter().map(|&v| if v { '#' } else { '.' })
                                  .collect::<String>());
    }
}

