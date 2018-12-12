use advent::AdventSolver;
use failure::Error;

const SERIAL_NO: i64 = 4172;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        println!("Max 3x3 square: {:?}",
                 find_largest_total_power(SERIAL_NO, 3, 3));
        println!("Max NxN square: {:?}",
                 find_largest_total_power(SERIAL_NO, 1, 300));
        Ok(())
    }
}

// Power in the single cell specified
fn cell_power_level(serial_no: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_no;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;
    power_level
}

fn find_largest_total_power(serial_no: i64, size_min: i64, size_max: i64)
        -> (i64, i64, i64) {
    let mut result = (0, 0, 0);
    let mut max_power: i64 = std::i64::MIN;
    for size in size_min..=size_max {
        eprint!(".");
        for x in 1..=300-size+1 {
            for y in 1..=300-size+1 {
                let power = square_power_level(serial_no, x, y, size);
                if power > max_power {
                    result = (x, y, size);
                    max_power = power;
                }
            }

        }
    }
    eprintln!("");
    result
}

// Power in the (size x size) square specified. In the spirit of Advent, uses
// the `cached` crate to memoize the results across invocations. (Without the
// cache, the naive solution runs in about 3 minutes. With the cache, 8 sec.)
cached! {
    SQUARE_POWER_LEVELS;
    fn square_power_level(serial_no: i64, x: i64, y: i64, size: i64) -> i64 = {
        if size == 1 {
            cell_power_level(serial_no, x, y)
        } else {
            square_power_level(serial_no, x, y, size-1) +
            // Count the bottom row of the square
            (x..x+size).map(|x| cell_power_level(serial_no, x, y+size-1))
                       .sum::<i64>() +
            // ...and the right row of the square, but don't count the bottom
            // right corner cell twice!
            (y..y+size-1).map(|y| cell_power_level(serial_no, x+size-1, y))
                         .sum::<i64>()
        }
    }
}

