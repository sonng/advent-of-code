pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
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
