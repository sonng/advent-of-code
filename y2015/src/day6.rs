use anyhow::Result;
use num::{BigUint, Zero};
use std::{cmp::max, fs};

pub fn execute() -> Result<()> {
    let input = fs::read_to_string("./inputs/6.txt")?;
    let input = input
        .split('\n')
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    println!("# Day 6");
    println!("Part 1: {:?}", solve_part_1(&input));
    println!("Part 2: {:?}", solve_part_2(&input));
    Ok(())
}

struct Instruction {
    action: Action,
    left_bot: Coord,
    right_top: Coord,
}

struct Coord {
    x: usize,
    y: usize,
}

enum Action {
    ON,
    TOOGLE,
    OFF,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut cursor = 0;
        let split: Vec<&str> = input.split(' ').collect();
        let action = match split[cursor] {
            "turn" => {
                cursor += 1;
                match split[cursor] {
                    "on" => Action::ON,
                    "off" => Action::OFF,
                    _ => unreachable!("not meant to happen!"),
                }
            }
            "toggle" => Action::TOOGLE,
            _ => unreachable!("not meant to happen!!"),
        };

        cursor += 1;

        let left_coord = Coord::from(split[cursor]);

        cursor += 2; // skipping the 'through'

        let right_coord = Coord::from(split[cursor]);

        Instruction {
            action: action,
            left_bot: left_coord,
            right_top: right_coord,
        }
    }
}

impl From<&str> for Coord {
    fn from(input: &str) -> Self {
        let split: Vec<&str> = input.split(',').collect();

        if split.len() != 2 {
            unreachable!("coord: weird input {:?}", input);
        }

        Coord {
            x: split[0].parse::<usize>().unwrap(),
            y: split[1].parse::<usize>().unwrap(),
        }
    }
}

fn solve_part_1(input: &Vec<Instruction>) -> usize {
    let mut grid = [[false; 1000]; 1000];

    input.iter().for_each(|inst| {
        for i in inst.left_bot.x..=inst.right_top.x {
            for j in inst.left_bot.y..=inst.right_top.y {
                grid[i][j] = match inst.action {
                    Action::ON => true,
                    Action::OFF => false,
                    Action::TOOGLE => !grid[i][j],
                }
            }
        }
    });

    grid.iter()
        .map(|row| {
            row.iter()
                .map(|col| if *col { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn solve_part_2(input: &Vec<Instruction>) -> BigUint {
    let mut grid: Vec<Vec<BigUint>> = vec![];

    for i in 0..1000 {
        grid.push(vec![]);
        for _j in 0..1000 {
            grid[i].push(BigUint::zero());
        }
    }

    input.iter().for_each(|inst| {
        for i in inst.left_bot.x..=inst.right_top.x {
            for j in inst.left_bot.y..=inst.right_top.y {
                match inst.action {
                    Action::ON => grid[i][j] += BigUint::from(1 as usize),
                    Action::OFF => {
                        if grid[i][j] == BigUint::from(0 as usize) {
                            grid[i][j] = BigUint::from(0 as usize)
                        } else {
                            grid[i][j] -= BigUint::from(1 as usize)
                        }
                    }
                    Action::TOOGLE => grid[i][j] += BigUint::from(2 as usize),
                }
            }
        }
    });

    let mut r = BigUint::zero();

    grid.iter().for_each(|row| {
        row.iter().for_each(|col| {
            r += col;
        });
    });

    r
}
