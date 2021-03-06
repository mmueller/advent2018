extern crate argparse;
#[macro_use] extern crate cached;
#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate failure;
extern crate image;
extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate num;
extern crate rand;
extern crate regex;

#[macro_use]
mod util;
mod advent;

use argparse::{ArgumentParser, StoreOption};

fn main() {
    let mut day: Option<usize> = None;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Advent of Code 2018");
        parser.refer(&mut day)
              .add_option(&["-d", "--day"], StoreOption,
                          "number of challenge to run");
        parser.parse_args_or_exit();
    }
    match day {
        Some(ref day) => {
            match advent::solve(*day) {
                Ok(_) => {},
                Err(e) => println!("error: {}", e)
            }
        },
        None => println!("--day is required"),
    }
}
