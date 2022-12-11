use std::fs;

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day11.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .filter_map(|c| Monkey::try_from(c).ok())
        .collect();
    println!("Day 11-1: {:?}", monkeys);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 11-2: {:}", "");
    Ok(())
}

#[derive(Debug)]
struct Monkey {
    name: String,
    items: Vec<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
struct Test {
    divisible: usize,
    true_case: usize,
    false_case: usize,
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
}

const NAME_INDEX: usize = 0;
const ITEMS_INDEX: usize = 1;
const OPERATION_INDEX: usize = 2;
const TEST_INDEX: usize = 3;

impl TryFrom<&str> for Operation {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.trim().split(' ').collect();

        let value = if split[5] == "old" {
            2
        } else {
            split[5].parse()?
        };
        match split[4] {
            "*" => Ok(Operation::Multiply(value)),
            "+" => {
                if split[5] == "old" {
                    Ok(Operation::Multiply(value))
                } else {
                    Ok(Operation::Add(value))
                }
            }
            _ => Err(Errors::ParseError("Operation parse error".into())),
        }
    }
}

impl TryFrom<Vec<String>> for Test {
    type Error = Errors;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(Errors::ParseError("Invalid lines for test".into()));
        }

        let divisible = value[0]
            .split(' ')
            .last()
            .ok_or(Errors::ParseError("Invalid divible".into()))?
            .parse::<usize>()?;
        let true_case = value[1]
            .split(' ')
            .last()
            .ok_or(Errors::ParseError("Invalid True case".into()))?
            .parse::<usize>()?;
        let false_case = value[2]
            .split(' ')
            .last()
            .ok_or(Errors::ParseError("Invalid False case".into()))?
            .parse::<usize>()?;

        Ok(Test {
            divisible,
            true_case,
            false_case,
        })
    }
}

impl TryFrom<&str> for Monkey {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split('\n').collect();

        if split.len() != 6 {
            return Err(Errors::ParseError("Invalid Monkey".into()));
        }

        let name = split[NAME_INDEX].split(' ').collect::<Vec<&str>>()[1]
            .trim_end_matches(':')
            .to_string();

        let items = split[ITEMS_INDEX].split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .filter_map(|n| n.trim().parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let operation = Operation::try_from(split[OPERATION_INDEX])?;

        let test = &split[TEST_INDEX..];
        let test: Vec<String> = test.iter().map(|c| c.to_string()).collect();
        let test = Test::try_from(test)?;

        Ok(Monkey {
            name,
            items,
            operation,
            test,
        })
    }
}
