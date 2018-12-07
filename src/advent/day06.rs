use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver;

// I don't like this one. Not cleaning it up. 🤯

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let coords = Self::read_coordinates()?;
        let grid = Self::make_grid(&coords);
        let mut areas = coords.iter()
                              .map(|_| 0)
                              .collect::<Vec<usize>>();
        let mut finite_areas: HashSet<usize> =
            (0..coords.len()).collect();

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                match grid[x][y] {
                    Some(index) => {
                        areas[index] += 1;
                        // My theory: Any area that reaches the bounding box
                        // will go on forever.
                        if x == 0 || y == 0 || x == grid.len()-1
                                            || y == grid[0].len()-1 {
                            finite_areas.remove(&index);
                        }
                    },
                    None => {}
                }
            }
        }

        let largest_finite_area =
            areas.iter()
                 .enumerate()
                 .filter(|&(index, _area)| finite_areas.contains(&index))
                 .max_by_key(|&(_index, area)| area)
                 .unwrap();
        println!("Largest finite area: coords[{}] {:?}: {}",
                 largest_finite_area.0, coords[largest_finite_area.0],
                 largest_finite_area.1);

        // Part 2: Count positions with < 10000 total distance to coords
        let mut found_something = true;
        let mut min_x: i32 = (grid.len()/2) as i32;
        let mut max_x: i32 = (grid.len()/2) as i32;
        let mut min_y: i32 = (grid[0].len()/2) as i32;
        let mut max_y: i32 = (grid[0].len()/2) as i32;
        let mut region_size: usize = 0;
        while found_something {
            found_something = false;
            for x in min_x..=max_x {
                let range = if x == min_x || x == max_x {
                    (min_y..=max_y).collect()
                } else {
                    vec![min_y, max_y]
                };
                for y in range {
                    let sum = coords.iter()
                                    .map(|&coord| {
                                        Self::manhattan_distance((x, y), coord)
                                    })
                                    .sum::<u32>();
                    if sum < 10000 {
                        found_something = true;
                        region_size += 1;
                    }
                }
            }
            min_x -= 1;
            max_x += 1;
            min_y -= 1;
            max_y += 1;
            eprint!("\r{} {} {}", region_size, min_x, min_y);
        }
        eprint!("\r");
        println!("Area with locations with total distance < 10k: {}",
                 region_size);
        Ok(())
    }
}

impl Solver {
    // Grid returned as a two-dim row ordered Vec<Vec>> where each item is
    // either Some(index of closest point in coords) or None (if there is no
    // unique closest point). Grid's (0, 0) is (min_x, min_y).
    fn make_grid(coords: &Vec<(i32, i32)>) -> Vec<Vec<Option<usize>>> {
        let min_x = coords.iter().min_by_key(|c| c.0).unwrap().0;
        let max_x = coords.iter().max_by_key(|c| c.0).unwrap().0;
        let min_y = coords.iter().min_by_key(|c| c.1).unwrap().1;
        let max_y = coords.iter().max_by_key(|c| c.1).unwrap().1;
        let mut result: Vec<Vec<Option<usize>>> = Vec::new();
        for x in min_x..=max_x {
            let mut row = Vec::new();
            for y in min_y..=max_y {
                row.push(Self::nearest_point((x, y), coords));
            }
            result.push(row);
        }
        result
    }

    fn nearest_point(point: (i32, i32),
                     coords: &Vec<(i32, i32)>) -> Option<usize> {
        let mut min_distance: Option<u32> = None;
        let mut nearest_indexes: Vec<usize> = Vec::new();
        for (index, coord) in coords.iter().enumerate() {
            let dist = Self::manhattan_distance(point, *coord);
            if min_distance.is_none() || dist < min_distance.unwrap() {
                min_distance = Some(dist);
                nearest_indexes.clear();
                nearest_indexes.push(index);
            } else if dist == min_distance.unwrap() {
                nearest_indexes.push(index);
            }
        }
        if nearest_indexes.len() > 1 {
            None
        } else {
            Some(nearest_indexes[0])
        }
    }

    fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
        ((p1.0-p2.0).abs() + (p1.1-p2.1).abs()) as u32
    }

    fn read_coordinates() -> Result<Vec<(i32, i32)>, Error> {
        let re = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
        BufReader::new(File::open("input/day06.txt")?)
                  .lines()
                  .collect::<Result<Vec<String>, _>>()?
                  .iter()
                  .map(|line| {
                      match re.captures(line) {
                          Some(caps) => Ok((caps["x"].parse::<i32>().unwrap(),
                                            caps["y"].parse::<i32>().unwrap())),
                          None => Err(format_err!("Parse error: {}", line)),
                      }
                  })
                  .collect::<Result<Vec<(i32,i32)>, _>>()
    }
}
