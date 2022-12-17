use std::{collections::HashSet, fs};

use anyhow::{Ok, Result};

use crate::Coord;

pub fn exec() -> Result<()> {
    let input_example = fs::read_to_string("./inputs/day17_example.txt")?;
    solve_part_1(&input_example)?;
    solve_part_2(&input_example)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    println!("Day 17-1: {}", "");
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 17-2: {}", "");
    Ok(())
}

#[derive(Debug)]
struct Shape(HashSet<Coord>);
