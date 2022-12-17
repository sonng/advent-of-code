pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::{num::ParseIntError, str::Utf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
enum Errors {
    #[error("Parsing Error: `{0}`")]
    ParseError(String),
    #[error("Parsing Int Error: `{0}`")]
    ParseIntError(#[from] ParseIntError),
    #[error("Parsing Utf8 Error: `{0}`")]
    ParseUtf8Error(#[from] Utf8Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }

    fn add(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn is_on_same_axis(&self, other: &Coord) -> bool {
        self.x == other.x || self.y == other.y
    }

    fn direction_towards(&self, other: &Coord) -> Coord {
        Coord {
            x: if self.x == other.x {
                0
            } else {
                if self.x < other.x {
                    1
                } else {
                    -1
                }
            },
            y: if self.y == other.y {
                0
            } else {
                if self.y < other.y {
                    1
                } else {
                    -1
                }
            },
        }
    }
}
