use std::{collections::BinaryHeap, fs};

use anyhow::Ok;

pub fn exec() -> anyhow::Result<()> {
    let input = fs::read_to_string("./inputs/day1.txt")?;
    let input = &input[..];

    solve_part1(input)?;
    solve_part2(input)?;

    Ok(())
}

fn calculate_score(input: &str) -> u64 {
    input.split('\n').map(|i| i.parse::<u64>().unwrap()).sum()
}

fn solve_part1(input: &str) -> anyhow::Result<()> {
    let result = input.split("\n\n").map(calculate_score).max();
    println!("Day 1-1: {}", result.unwrap());

    Ok(())
}

fn solve_part2(input: &str) -> anyhow::Result<()> {
    let mut result = input
        .split("\n\n")
        .map(calculate_score)
        .collect::<BinaryHeap<u64>>();

    let mut total = 0;
    for _ in 0..3 {
        total += result.pop().unwrap();
    }

    println!("Day 1-2: {}", total);

    Ok(())
}
