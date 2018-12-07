use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

#[derive(Default)]
struct Worker {
    task: Option<char>,
    time_remaining: u32,
}

impl Worker {
    fn assign(&mut self, task: char, overhead: u32) {
        assert!(self.task.is_none());
        self.task = Some(task);
        self.time_remaining = Self::task_time(task, overhead);
    }

    fn is_busy(&self) -> bool {
        self.task.is_some()
    }

    // Work for the given number of seconds. Returns Some(task) if the task
    // was completed, otherwise None.
    fn work(&mut self, elapsed: u32) -> Option<char> {
        if self.time_remaining <= elapsed {
            self.time_remaining = 0;
            self.task.take()
        } else {
            self.time_remaining -= elapsed;
            None
        }
    }

    fn task_time(task: char, overhead: u32) -> u32 {
        (task as u32) - 64 + overhead
    }
}

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let instructions = Self::read_instructions()?;
        println!("Instruction sequence (solo project): {}",
                 Self::build_sleigh(&instructions, 1, 0).0);
        println!("Time to complete with 5 workers: {}",
                 Self::build_sleigh(&instructions, 5, 60).1);
        Ok(())
    }
}

impl Solver {
    fn build_sleigh(instructions: &Vec<(char, char)>,
                    num_workers: usize, step_overhead: u32) -> (String, u32) {
        // Some steps may have no dependencies, so we'll only see them on the
        // left-hand side, and others will have no steps that depend on them,
        // so we'll only see them on the right-hand side. Collect them all.
        let mut available_steps = HashSet::new();
        for instruction in instructions.iter() {
            available_steps.insert(instruction.0);
            available_steps.insert(instruction.1);
        }
        let mut done_steps = Vec::new();
        let mut workers: Vec<Worker> = iter::repeat_with(Worker::default)
                                            .take(num_workers)
                                            .collect();

        // Build dependency graph
        let mut depgraph: HashMap<char, Vec<char>> = HashMap::new();
        for instruction in instructions {
            let v = depgraph.entry(instruction.1).or_insert(Vec::new());
            v.push(instruction.0);
        }

        // Loop steps forward one second at a time, assigning workers tasks
        // whenever tasks and workers are available.
        let mut seconds_elapsed: u32 = 0;
        while !available_steps.is_empty() ||
              workers.iter().any(|worker| worker.is_busy()) {

            // Determine work available
            let mut ready_steps: Vec<char> = Vec::new();
            for step in available_steps.iter() {
                match depgraph.get(&step) {
                    Some(prereqs) => {
                        if prereqs.iter().all(|s| done_steps.contains(s)) {
                            ready_steps.push(*step);
                        }
                    },
                    None => {
                        ready_steps.push(*step);
                    }
                }
            }

            // Assign work
            ready_steps.sort();
            for worker in workers.iter_mut() {
                if !worker.is_busy() && ready_steps.len() > 0 {
                    let next_step = ready_steps.remove(0);
                    worker.assign(next_step, step_overhead);
                    available_steps.remove(&next_step);
                }
            }

            // Do work
            for worker in workers.iter_mut() {
                match worker.work(1) {
                    Some(task) => done_steps.push(task),
                    None => {}
                }
            }
            seconds_elapsed += 1;
        }
        (done_steps.iter().collect(), seconds_elapsed)
    }

    // Returns a vector of dependency tuples (a, b), where step A must be done
    // before step B can begin.
    fn read_instructions() -> Result<Vec<(char, char)>, Error> {
        let re = Regex::new(
            r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.")?;
        BufReader::new(File::open("input/day07.txt")?)
                  .lines()
                  .collect::<Result<Vec<String>, _>>()?
                  .iter()
                  .map(|line| {
                      match re.captures(line) {
                          Some(caps) => {
                              Ok((caps[1].chars().nth(0).unwrap(),
                                  caps[2].chars().nth(0).unwrap()))
                          },
                          None => Err(format_err!("Parse error: {}", line))
                      }
                  })
                  .collect::<Result<Vec<(char, char)>, _>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;

    lazy_static! {
        static ref DEPS: Vec<(char, char)> =
            vec![('C', 'A'), ('C', 'F'), ('A', 'B'), ('A', 'D'),
                 ('B', 'E'), ('D', 'E'), ('F', 'E')];
    }

    #[test]
    fn part1_example() {
        let (build_order, _) = Solver::build_sleigh(&DEPS, 1, 0);
        assert_eq!("CABDFE", build_order);
    }

    #[test]
    fn part2_example() {
        let (_, time_spent) = Solver::build_sleigh(&DEPS, 2, 0);
        assert_eq!(15, time_spent);
    }
}
