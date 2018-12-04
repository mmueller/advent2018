use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver;

lazy_static! {
    static ref SHIFT_ENTRY_REGEX: Regex = Regex::new(
        r"(?x)
          :(?P<min>\d\d)\]\s
          (?P<log>
            Guard\s\#(?P<guard>\d+)\sbegins\sshift
           |falls\sasleep
           |wakes\sup)").unwrap();
}

struct Guard {
    id: usize,
    sleepy_minutes: [u32; 60],
}

impl Guard {
    fn new(id: usize) -> Guard {
        Guard {
            id: id,
            sleepy_minutes: [0; 60]
        }
    }

    fn sleepiest_minute(&self) -> u8 {
        self.sleepy_minutes
            .iter()
            .enumerate()
            .max_by_key(|&(_min, count)| count)
            .unwrap().0 as u8
    }

    fn total_sleep_time(&self) -> u32 {
        self.sleepy_minutes.iter().sum()
    }
}

#[derive(Debug)]
enum ShiftEntry {
    ShiftStart { guard_id: usize },
    FallsAsleep { guard_id: usize, minute: u8 },
    WakesUp { guard_id: usize, minute: u8 },
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let guards = Self::load_guard_data()?;

        // Part 1: Guard who sleeps the most.
        let sleepiest_guard =
            guards.iter()
                  .max_by_key(|&guard| guard.total_sleep_time())
                  .unwrap();
        println!("Part 1: Guard {}'s sleepiest minute: {}",
                 sleepiest_guard.id,
                 sleepiest_guard.sleepiest_minute());

        // Part 2: Guard who sleeps the most at a particular minute.
        let better_target =
            guards.iter()
                  .map(|guard| {
                      let m = guard.sleepiest_minute();
                      (guard, m, guard.sleepy_minutes[m as usize])
                  })
                  .max_by_key(|&(_guard, _min, count)| count)
                  .unwrap().0;
        println!("Part 2: Guard {}'s sleepiest minute: {}",
                 better_target.id, better_target.sleepiest_minute());

        Ok(())
    }
}

impl Solver {
    fn load_guard_data() -> Result<Vec<Guard>, Error> {
        let mut guards = HashMap::new();
        let shifts = Self::read_shifts()?;
        let mut fell_asleep: u8 = 0;
        for shift_line in shifts {
            match shift_line {
                ShiftEntry::ShiftStart { guard_id: _ } => {
                },
                ShiftEntry::FallsAsleep { guard_id: _, minute } => {
                    fell_asleep = minute;
                },
                ShiftEntry::WakesUp { guard_id, minute } => {
                    let guard = guards.entry(guard_id)
                                      .or_insert(Guard::new(guard_id));
                    for m in fell_asleep..minute {
                        (*guard).sleepy_minutes[m as usize] += 1;
                    }
                }
            }
        }
        // Temporary `result` is apparently needed to make borrow checker happy.
        let result = guards.drain()
                           .map(|(_, v)| v)
                           .collect::<Vec<Guard>>();
        Ok(result)
    }

    fn read_shifts() -> Result<Vec<ShiftEntry>, Error> {
        let mut lines =
            BufReader::new(File::open("input/day04.txt")?)
                      .lines()
                      .collect::<Result<Vec<String>, _>>()?;
        // Naive string sort is sufficient to get log entries in order.
        lines.sort();

        let mut guard_id: usize = 0;
        lines.iter()
             .map(|line| {
                 match SHIFT_ENTRY_REGEX.captures(&line) {
                     Some(caps) => {
                         let min = caps["min"].parse::<u8>()?;
                         match &caps["log"] {
                             "falls asleep" => {
                                 Ok(ShiftEntry::FallsAsleep {
                                     guard_id: guard_id,
                                     minute: min
                                 })
                             },
                             "wakes up" => {
                                 Ok(ShiftEntry::WakesUp {
                                     guard_id: guard_id,
                                     minute: min
                                 })
                             },
                             _ => {
                                 guard_id = caps["guard"].parse::<usize>()?;
                                 Ok(ShiftEntry::ShiftStart {
                                     guard_id: guard_id
                                 })
                             }
                         }
                     },
                     None => Err(format_err!("Couldn't parse: {}", line))
                 }
             })
             .collect::<Result<Vec<ShiftEntry>, _>>()
    }
}
