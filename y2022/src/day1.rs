use core::num;
use std::{cmp::max, collections::BinaryHeap, fs};

use anyhow::Ok;

pub fn exec() -> anyhow::Result<()> {
    let input = fs::read("./inputs/day1.txt")?;
    let input = &input[..];

    solve_part1(input)?;
    solve_part2(input)?;

    Ok(())
}

fn solve_part1(input: &[u8]) -> anyhow::Result<()> {
    let mut index = 0;
    let mut total = 0;
    let mut max_score = 0;
    while index < input.len() {
        if input[index] == b'\n' {
            index += 1;
            max_score = max(max_score, total);
            total = 0;
            continue;
        }

        let mut new_line_index = index;
        while new_line_index < input.len() && input[new_line_index] != b'\n' {
            new_line_index += 1;
        }

        let number = String::from_utf8(input[index..new_line_index].to_vec())?;
        let number = number.parse::<u32>()?;

        total += number;

        index = new_line_index + 1;
    }

    println!("Day 1-1: {}", max_score);

    Ok(())
}

fn solve_part2(input: &[u8]) -> anyhow::Result<()> {
    let mut index = 0;
    let mut total = 0;
    let mut heap = BinaryHeap::new();
    while index < input.len() {
        if input[index] == b'\n' {
            index += 1;
            heap.push(total);
            total = 0;
            continue;
        }

        let mut new_line_index = index;
        while new_line_index < input.len() && input[new_line_index] != b'\n' {
            new_line_index += 1;
        }

        let number = String::from_utf8(input[index..new_line_index].to_vec())?;
        let number = number.parse::<u32>()?;

        total += number;

        index = new_line_index + 1;
    }

    let mut total = 0;
    for _ in 0..3 {
        total += heap.pop().unwrap();
    }

    println!("Day 1-2: {}", total);

    Ok(())
}
