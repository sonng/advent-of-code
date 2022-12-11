use std::fs;

use anyhow::Result;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day11.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    println!("Day 11-1: {:}", "");
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 11-2: {:}", "");
    Ok(())
}
