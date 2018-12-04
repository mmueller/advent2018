use advent::AdventSolver;
use failure::Error;
use rand;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

lazy_static! {
    static ref CLAIM_RE: Regex =
        Regex::new(r"(?x)
                   ^\#(?P<id>\d+)\s@\s
                   (?P<x>\d+),(?P<y>\d+):\s
                   (?P<w>\d+)x(?P<h>\d+)$").unwrap();
}

#[derive(Clone)]
struct Claim {
    id: usize,
    pos_x: usize,
    pos_y: usize,
    width: usize,
    height: usize,
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Claim #{} @ {},{}: {}x{}>",
               self.id, self.pos_x, self.pos_y, self.width, self.height)
    }
}

impl Claim {
    fn parse(line: &str) -> Result<Claim, Error> {
        match CLAIM_RE.captures(line) {
            Some(caps) => {
                Ok(Claim {
                    id:     caps["id"].parse::<usize>()?,
                    pos_x:  caps["x"].parse::<usize>()?,
                    pos_y:  caps["y"].parse::<usize>()?,
                    width:  caps["w"].parse::<usize>()?,
                    height: caps["h"].parse::<usize>()?,
                })
            },
            None => Err(format_err!("Couldn't parse claim: {}", line)),
        }
    }

    fn squares_covered(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for i in self.pos_x..self.pos_x+self.width {
            for j in self.pos_y..self.pos_y+self.height {
                result.push((i, j));
            }
        }
        result
    }
}

#[derive(PartialEq,Eq)]
enum SquareState {
    Empty,
    SingleCoverage(usize), // Parameter is the id of the claim covering
    MultipleCoverage,
}

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let claims = Self::read_claims()?;

        // Uncomment if you want.
        //Self::write_animation(&claims)?;

        // Build coverage map
        let mut coverage: HashMap<(usize, usize), SquareState> = HashMap::new();
        for claim in &claims {
            for &pos in claim.squares_covered().iter() {
                let state = coverage.entry(pos).or_insert(SquareState::Empty);
                *state = match state {
                    SquareState::Empty =>
                        SquareState::SingleCoverage(claim.id),
                    SquareState::SingleCoverage(_) =>
                        SquareState::MultipleCoverage,
                    SquareState::MultipleCoverage =>
                        SquareState::MultipleCoverage,
                };
            }
        }

        // Find squares covered by multiple claims
        let squares_covered_by_2_or_more_claims =
            coverage.iter()
                    .filter(|&(_, state)|
                            *state == SquareState::MultipleCoverage)
                    .count();
        println!("Squares covered by 2 or more claims: {}",
                 squares_covered_by_2_or_more_claims);

        // Find the only claim that is uncompromised
        let uncompromised_claims =
            claims.iter()
                  .filter(|claim| {
                      claim.squares_covered()
                           .iter()
                           .all(|pos| coverage[pos] ==
                                      SquareState::SingleCoverage(claim.id))
                  });
        println!("Uncompromised claims:");
        for claim in uncompromised_claims {
            println!("{}", claim);
        }
        Ok(())
    }
}

impl Solver {
    fn read_claims() -> Result<Vec<Claim>, Error> {
        BufReader::new(File::open("input/day03.txt")?)
                  .lines()
                  .collect::<Result<Vec<String>, _>>()?
                  .iter()
                  .map(|line| Claim::parse(line))
                  .collect()
    }

    // Hacky animation for funsies
    #[allow(dead_code)]
    fn write_animation(claims: &Vec<Claim>) -> Result<(), Error> {
        let mut buf: [u8;1000*1000*4] = [0;1000*1000*4];
        for x in 0..1000 {
            for y in 0..1000 {
                buf[y*4000+x*4+3] = 0xff;
            }
        }

        for claim in claims {
            let r = rand::random::<u8>();
            let g = rand::random::<u8>();
            let b = rand::random::<u8>();
            for (x, y) in claim.squares_covered() {
                buf[y*4000+x*4] = r;
                buf[y*4000+x*4+1] = g;
                buf[y*4000+x*4+2] = b;
            }
            image::save_buffer(
                &Path::new(&format!("imgs/claim{:04}.png", claim.id)),
                &buf, 1000, 1000, image::RGBA(8))?;
        }
        Ok(())
    }
}
