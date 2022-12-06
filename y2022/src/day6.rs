use std::{collections::HashSet, fs};
use thiserror::Error;

use anyhow::Result;

#[derive(Error, Debug)]
enum Errors {
    #[error("the input is not valid")]
    InvalidInput,
}

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day6.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let results = input.as_bytes();

    if results.len() <= 3 {
        return Err(Errors::InvalidInput.into());
    }

    let results = results
        .windows(4)
        .position(|letters| HashSet::<u8>::from_iter(letters.iter().map(|c| *c)).len() == 4)
        .unwrap();
    println!("Day 6-1: {}", results + 4);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let results = input.as_bytes();

    if results.len() <= 13 {
        return Err(Errors::InvalidInput.into());
    }

    let results = results
        .windows(14)
        .position(|letters| HashSet::<u8>::from_iter(letters.iter().map(|c| *c)).len() == 14)
        .unwrap();
    println!("Day 6-2: {}", results + 14);
    Ok(())
}
