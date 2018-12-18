use advent::AdventSolver;
use failure::Error;
use num::FromPrimitive;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
#[allow(unused_imports)]
use std::{thread, time};

#[derive(Default)]
pub struct Solver;

#[derive(Clone,Copy)]
enum MapCell {
    Empty,
    VerticalTrack,
    HorizontalTrack,
    Intersection,
    CornerTL,
    CornerTR,
    CornerBL,
    CornerBR,
}

enum_from_primitive! {
    #[derive(Clone,Copy,Debug,PartialEq)]
    enum Direction {
        Up = 0,
        Right = 1,
        Down = 2,
        Left = 3,
    }
}

enum_from_primitive! {
    #[derive(Clone,Copy,Debug,PartialEq)]
    enum RelativeDirection {
        Left = -1,
        Straight = 0,
        Right = 1,
    }
}

#[derive(Debug)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    dir: Direction,
    next_turn: RelativeDirection,
}

struct Map {
    map_data: Vec<MapCell>,
    width: usize,
    height: usize,
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, dir: Direction) -> Cart {
        Cart {
            id: id,
            x: x,
            y: y,
            dir: dir,
            next_turn: RelativeDirection::Left,
        }
    }

    // Move the cart by 1 tick and possibly turn.
    fn step(&mut self, map: &Map) {
        // We always leave the cart pointing in the direction it will move on
        // its next move, so obey direction naively here.
        match self.dir {
            Direction::Up =>    { self.y -= 1; },
            Direction::Right => { self.x += 1; },
            Direction::Down =>  { self.y += 1; },
            Direction::Left =>  { self.x -= 1; },
        }

        // Some locations will cause the cart to turn, do so now.
        match map.at(self.x, self.y) {
            // No turns
            MapCell::VerticalTrack | MapCell::HorizontalTrack => {
            },
            // Cart naturally turns at corners
            MapCell::CornerTL => {
                if self.dir == Direction::Up {
                    self.dir = Direction::Right;
                } else {
                    self.dir = Direction::Down;
                }
            },
            MapCell::CornerTR => {
                if self.dir == Direction::Right {
                    self.dir = Direction::Down;
                } else {
                    self.dir = Direction::Left;
                }
            },
            MapCell::CornerBL => {
                if self.dir == Direction::Left {
                    self.dir = Direction::Up;
                } else {
                    self.dir = Direction::Right;
                }
            },
            MapCell::CornerBR => {
                if self.dir == Direction::Down {
                    self.dir = Direction::Left;
                } else {
                    self.dir = Direction::Up;
                }
            },
            // Rotation logic applies
            MapCell::Intersection => {
                self.turn();
            },
            MapCell::Empty => {
                panic!();
            }
        }
    }

    // Rotates the cart according to the rules, but does not move it.
    fn turn(&mut self) {
        self.dir =
            Direction::from_i32(
                modulo(self.dir as i32 + self.next_turn as i32, 4)).unwrap();
        self.next_turn =
            RelativeDirection::from_i32(
                modulo(self.next_turn as i32 + 2, 3) - 1).unwrap();
    }
}

impl Map {
    fn from_text(lines: &Vec<String>) -> Result<(Map, Vec<Cart>), Error> {
        let height = lines.len();
        let width = lines[0].len();
        let mut map_data = Vec::new();
        let mut carts = Vec::new();

        enum State {
            Normal,
            InBox,
        }

        for (y, line) in lines.iter().enumerate() {
            let mut state = State::Normal;
            for (x, c) in line.chars().enumerate() {
                let next_id = carts.len();
                map_data.push(match c {
                    ' ' => MapCell::Empty,
                    '|' => MapCell::VerticalTrack,
                    '-' => MapCell::HorizontalTrack,
                    '+' => MapCell::Intersection,
                    '/' => {
                        match state {
                            State::Normal => {
                                state = State::InBox;
                                MapCell::CornerTL
                            },
                            State::InBox => {
                                state = State::Normal;
                                MapCell::CornerBR
                            },
                        }
                    },
                    '\\' => {
                        match state {
                            State::Normal => {
                                state = State::InBox;
                                MapCell::CornerBL
                            },
                            State::InBox => {
                                state = State::Normal;
                                MapCell::CornerTR
                            },
                        }
                    },
                    // Locations with carts
                    '>' => {
                        carts.push(Cart::new(next_id, x, y, Direction::Right));
                        MapCell::HorizontalTrack
                    },
                    '<' => {
                        carts.push(Cart::new(next_id, x, y, Direction::Left));
                        MapCell::HorizontalTrack
                    },
                    '^' => {
                        carts.push(Cart::new(next_id, x, y, Direction::Up));
                        MapCell::VerticalTrack
                    },
                    'v' => {
                        carts.push(Cart::new(next_id, x, y, Direction::Down));
                        MapCell::VerticalTrack
                    },
                    _ => return Err(format_err!("Invalid map char: {}", c))
                });
            }
        }

        let map = Map {
            map_data: map_data,
            width: width,
            height: height,
        };
        Ok((map, carts))
    }

    fn at(&self, x: usize, y: usize) -> MapCell {
        self.map_data[y*self.width + x]
    }

    #[allow(dead_code)]
    fn draw(&self, carts: &Vec<Cart>) {
        // Build a quick lookup table for carts on the map (by position)
        let mut cart_index: HashMap<(usize,usize),&Cart> = HashMap::new();
        for cart in carts.iter() {
            cart_index.insert((cart.x, cart.y), &cart);
        }

        // Draw the map
        print!("\x1b[2J");
        for y in 0..self.height {
            for x in 0..self.width {
                let cart = cart_index.get(&(x, y));
                let cell = self.at(x, y);
                let c = match cart {
                    Some(cart) => {
                        match cart.dir {
                            Direction::Up => '▲',
                            Direction::Right => '▶',
                            Direction::Down => '▼',
                            Direction::Left => '◀',
                        }
                    },
                    None => {
                        match cell {
                            MapCell::Empty => ' ',
                            MapCell::VerticalTrack => '│',
                            MapCell::HorizontalTrack => '─',
                            MapCell::Intersection => '┼',
                            MapCell::CornerTL => '┌',
                            MapCell::CornerTR => '┐',
                            MapCell::CornerBL => '└',
                            MapCell::CornerBR => '┘',
                        }
                    }
                };
                print!("{}", c);
            }
            println!("");
        }
    }
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let lines = BufReader::new(File::open("input/day13.txt")?)
                              .lines()
                              .collect::<Result<Vec<String>, _>>()?;
        let (map, mut carts) = Map::from_text(&lines)?;
        while carts.len() > 1 {
            carts.sort_by_key(|cart| (cart.y, cart.x));
            let mut i = 0;
            while i < carts.len() {
                carts[i].step(&map);
                if let Some(j) = Self::detect_collisions(&carts[i], &carts) {
                    println!("Crash occurred at ({}, {})",
                             carts[i].x, carts[i].y);
                    carts.remove(i);
                    let r = Self::index_of(j, &carts);
                    carts.remove(r);
                    if r < i {
                        i -= 1;
                    }
                } else {
                    i += 1;
                }
            }
            // Uncomment for cute animations
            //map.draw(&carts);
            //thread::sleep(time::Duration::from_millis(400));
        }
        println!("Last cart: {:?}", carts[0]);
        Ok(())
    }
}

impl Solver {
    // Returns the id of the cart that moving_cart collided with, or None if
    // there is no collision.
    fn detect_collisions(moving_cart: &Cart, carts: &Vec<Cart>)
            -> Option<usize> {
        for c in carts.iter() {
            if c.id != moving_cart.id &&
                    c.x == moving_cart.x && c.y == moving_cart.y {
                return Some(c.id);
            }
        }
        None
    }

    // Get the index of a cart in the carts Vec, given its id.
    fn index_of(id: usize, carts: &Vec<Cart>) -> usize {
        carts.iter().enumerate()
                    .find(|(_, cart)| cart.id == id)
                    .unwrap()
                    .0
    }
}

fn modulo(num: i32, modulus: i32) -> i32 {
    ((num % modulus) + modulus) % modulus
}
