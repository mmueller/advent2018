use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone,Copy,Debug)]
struct Point {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Point {
    // Get the position of a point at a given time.
    fn position_at_time(&self, t: i64) -> (i64, i64) {
        (self.px + t*self.vx, self.py + t*self.vy)
    }
}

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let points = Self::read_points()?;
        let mut connectedness_history: Vec<f64> = Vec::new();
        for t in 0.. {
            let connectedness = Self::measure_connectedness(&points, t);
            let stddev = Self::stddev(&connectedness_history);
            let mean: f64 = connectedness_history.iter().sum::<f64>() /
                            connectedness_history.len() as f64;
            // Magic numbers!
            if stddev > 0.01 && connectedness-mean > stddev*4.0 {
                println!("At t={}, connectedness is {} stddev above average!",
                         t, (connectedness-mean)/stddev);
                Self::draw_points(&points, t)?;
                break;
            }
            connectedness_history.push(connectedness);
        }
        Ok(())
    }
}

impl Solver {

    fn draw_points(points: &Vec<Point>, t: i64) -> Result<(), Error> {
        let points: Vec<(i64, i64)> = points.iter()
                                            .map(|p| p.position_at_time(t))
                                            .collect();
        let minx = points.iter().min_by_key(|&(x, _)| x).unwrap().0;
        let miny = points.iter().min_by_key(|&(_, y)| y).unwrap().1;
        let maxx = points.iter().max_by_key(|&(x, _)| x).unwrap().0;
        let maxy = points.iter().max_by_key(|&(_, y)| y).unwrap().1;
        let width = (maxx - minx + 1) as usize;
        let height = (maxy - miny + 1) as usize;
        let xoff = -minx;
        let yoff = -miny;
        if width*height > 100_000 {
            // Don't try to draw very large images
            return Err(format_err!("Image that big is probably not right."));
        }

        // Initialize buffer with zero RGB and 0xff alpha.
        let mut buf: Vec<u8> = Vec::with_capacity(width*height*4);
        for _x in 0..width {
            for _y in 0..height {
                buf.push(0x0);
                buf.push(0x0);
                buf.push(0x0);
                buf.push(0xff);
            }
        }

        // Write the points themselves
        for point in points.iter() {
            let x = (point.0 + xoff) as usize;
            let y = (point.1 + yoff) as usize;
            buf[y*4*width+x*4] = 0x40;
            buf[y*4*width+x*4+1] = 0xff;
            buf[y*4*width+x*4+2] = 0x40;
        }
        image::save_buffer(
            &Path::new("imgs/day10_message.png"),
            buf.as_slice(), width as u32, height as u32, image::RGBA(8))?;
        println!("Image saved to imgs/day10_message.png.");
        Ok(())
    }

    // Returns the average number of neighbors (max 4) of each point.
    fn measure_connectedness(points: &Vec<Point>, t: i64) -> f64 {
        let index: HashMap<(i64,i64), &Point> =
            points.iter()
                  .map(|p| (p.position_at_time(t), p))
                  .collect();
        points.iter()
              .map(|p| {
                  let (x, y) = p.position_at_time(t);
                  (index.contains_key(&(x-1, y)) as i64 +
                   index.contains_key(&(x+1, y)) as i64 +
                   index.contains_key(&(x, y-1)) as i64 +
                   index.contains_key(&(x, y+1)) as i64) as f64
              })
              .sum::<f64>() / points.len() as f64
    }

    fn read_points() -> Result<Vec<Point>, Error> {
        let re = Regex::new(
            r"(?x)
              position=<\s*(-?\d+),\s*(-?\d+)>\s
              velocity=<\s*(-?\d+),\s*(-?\d+)>")?;
        BufReader::new(File::open("input/day10.txt")?)
                  .lines()
                  .collect::<Result<Vec<String>, _>>()?
                  .iter()
                  .map(|line| {
                      match re.captures(line) {
                          Some(caps) => {
                              Ok(Point {
                                  px: caps[1].parse::<i64>()?,
                                  py: caps[2].parse::<i64>()?,
                                  vx: caps[3].parse::<i64>()?,
                                  vy: caps[4].parse::<i64>()?,
                              })
                          },
                          None => Err(format_err!("Parse error: {}", line))
                      }
                  })
                  .collect::<Result<Vec<Point>, _>>()
    }

    fn stddev(values: &Vec<f64>) -> f64 {
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        ((values.iter()
               .map(|v: &f64| (mean - *v).abs().powf(2.0))
               .sum::<f64>() / values.len() as f64) as f64).sqrt()
    }
}
