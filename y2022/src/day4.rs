use std::fs;

use anyhow::{Ok, Result};

pub fn exec() -> anyhow::Result<()> {
    let input = fs::read_to_string("./inputs/day4.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

struct Assignment {
    lower: u64,
    upper: u64,
}

impl TryFrom<&str> for Assignment {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split('-').collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(());
        }

        let lower: u64 = split[0].parse::<u64>().unwrap();
        let upper: u64 = split[1].parse::<u64>().unwrap();

        Result::Ok(Self { lower, upper })
    }
}

impl Assignment {
    fn is_inside(&self, other: &Assignment) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    fn is_overlap(&self, other: &Assignment) -> bool {
        (self.lower <= other.lower && self.upper >= other.lower)
            || (self.upper >= other.upper && self.lower <= other.upper)
    }
}

fn solve_part_1(input: &str) -> anyhow::Result<()> {
    let result = input
        .split('\n')
        .map(|line| {
            let split = line.split(',').collect::<Vec<&str>>();
            (
                Assignment::try_from(split[0]).unwrap(),
                Assignment::try_from(split[1]).unwrap(),
            )
        })
        .filter(|ass| ass.0.is_inside(&ass.1) || ass.1.is_inside(&ass.0))
        .count();
    println!("Day 4-1: {:?}", result);
    Ok(())
}

fn solve_part_2(input: &str) -> anyhow::Result<()> {
    let result = input
        .split('\n')
        .map(|line| {
            let split = line.split(',').collect::<Vec<&str>>();
            (
                Assignment::try_from(split[0]).unwrap(),
                Assignment::try_from(split[1]).unwrap(),
            )
        })
        .filter(|ass| ass.0.is_overlap(&ass.1) || ass.1.is_overlap(&ass.0))
        .count();
    println!("Day 4-2: {:?}", result);
    Ok(())
}
