use std::fs;

use anyhow::Result;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day10.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    println!("Day 10-1: {:?}", "");
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 10-2: {:?}", "");
    Ok(())
}
