use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};
use thiserror::Error;

use anyhow::Result;

#[derive(Error, Debug)]
enum Errors {
    #[error("the input is not valid")]
    InvalidInput,
}

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day6.txt")?;

    solve_part_1(&input, Instant::now())?;
    solve_part_1_2(&input, Instant::now())?;
    solve_part_2(&input, Instant::now())?;
    solve_part_2_2(&input, Instant::now())?;

    Ok(())
}

fn solve_part_1(input: &str, start: Instant) -> Result<()> {
    let results = input.as_bytes();

    if results.len() <= 3 {
        return Err(Errors::InvalidInput.into());
    }

    let results = results
        .windows(4)
        .position(|letters| HashSet::<u8>::from_iter(letters.iter().map(|c| *c)).len() == 4)
        .unwrap();
    println!(
        "Day 6-1: {} ({:?})",
        results + 4,
        start.elapsed().as_micros()
    );
    Ok(())
}

fn solve_part_1_2(input: &str, start: Instant) -> Result<()> {
    let higher_bound = hashmap_solve(input, 4);
    println!(
        "Day 6-1 (2): {} ({:?})",
        higher_bound,
        start.elapsed().as_micros()
    );
    Ok(())
}

fn solve_part_2(input: &str, start: Instant) -> Result<()> {
    let results = input.as_bytes();

    if results.len() <= 13 {
        return Err(Errors::InvalidInput.into());
    }

    let results = results
        .windows(14)
        .position(|letters| HashSet::<u8>::from_iter(letters.iter().map(|c| *c)).len() == 14)
        .unwrap();
    println!(
        "Day 6-2: {} ({:?})",
        results + 14,
        start.elapsed().as_micros()
    );
    Ok(())
}

fn solve_part_2_2(input: &str, start: Instant) -> Result<()> {
    let higher_bound = hashmap_solve(input, 14);
    println!(
        "Day 6-2 (2): {} ({:?})",
        higher_bound,
        start.elapsed().as_micros()
    );
    Ok(())
}

fn hashmap_solve(input: &str, cap: usize) -> usize {
    let input = input.as_bytes();

    let mut seen = HashMap::<u8, usize>::new();

    let mut lower_bound = 0;
    let mut higher_bound = 0;

    while higher_bound < input.len() {
        seen.entry(input[higher_bound])
            .and_modify(|c| *c += 1)
            .or_insert(1);

        if seen.len() == cap {
            return higher_bound + 1;
        }

        if higher_bound == lower_bound + cap - 1 {
            seen.entry(input[lower_bound])
                .and_modify(|c| *c -= 1)
                .or_insert(0);

            if seen.get(&input[lower_bound]).unwrap() == &0 {
                seen.remove(&input[lower_bound]);
            }

            lower_bound += 1;
        }

        higher_bound += 1;
    }

    higher_bound + 1
}
