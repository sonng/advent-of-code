use std::{collections::HashMap, fs, future::pending};

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day10.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input
        .split("\n")
        .filter_map(|c| Instruction::try_from(c).ok())
        .collect();

    let mut cpu = CPU::new();

    let mut results = vec![];

    cpu.exec(instructions, 220, |step, value| {
        println!(
            "DURING: s:{:?} r: {:?} = {:?}",
            step + 1,
            value,
            "" // (step + 1) as i64 * value
        );

        match step {
            19 | 59 | 99 | 139 | 179 | 219 => results.push((step + 1) as i64 * value),
            _ => {}
        }
    });
    println!("Results: {:?}", results);
    println!("Day 10-1: {:?}", results.iter().sum::<i64>());
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 10-2: {:?}", "");
    Ok(())
}

#[derive(Clone, Copy)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl TryFrom<&str> for Instruction {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(' ').collect();

        let cmd = split[0];

        match cmd {
            "addx" => {
                let value: i64 = split[1].parse()?;
                Ok(Self::Addx(value))
            }
            "noop" => Ok(Self::Noop),
            _ => Err(Errors::ParseError("invalid instruction".into())),
        }
    }
}

struct CPU {
    register: i64,
    step: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: 1,
            step: 0,
        }
    }

    fn peek(&self) -> i64 {
        self.register
    }

    fn exec<F>(&mut self, instructions: Vec<Instruction>, max_steps: usize, mut callback: F)
    where
        F: FnMut(usize, i64),
    {
        let mut instructions: Vec<&Instruction> = instructions.iter().rev().collect();
        let mut pending_instruction: Option<(usize, Instruction)> = None;
        for i in 0..max_steps {
            println!("BEFORE: s:{:?} r:{:?}", i + 1, self.register);
            callback(i, self.register);
            if let Some((when, instruction)) = pending_instruction {
                if i == when {
                    match instruction {
                        Instruction::Noop => {
                            println!(
                                "EXECUTE SCHEDULED NOOP: s:{:?} r:{:?}",
                                i + 1,
                                self.register
                            );
                            // println!("{:?}-{:?}: Noop", i, self.register);
                        }
                        Instruction::Addx(value) => {
                            println!("EXECUTE ADD: s:{:?} r:{:?}", i + 1, self.register);
                            self.register += value;
                            // println!(
                            //     "{:?}-{:?}: Executed {:?} -- {:?} -> {:?}",
                            //     i, self.register, when, value, self.register
                            // );
                        }
                    }
                    pending_instruction = None;
                } else {
                    // println!("{:?}-{:?}: Noop", i, self.register);
                }
            } else if let Some(instruction) = instructions.pop() {
                match instruction {
                    Instruction::Noop => {
                        println!("EXECUTE NOOP: s:{:?} r:{:?}", i + 1, self.register);
                        // println!("{:?}-{:?}: Noop", i, self.register);
                    }
                    Instruction::Addx(value) => {
                        // println!(
                        //     "{:?}-{:?}: Future {:?} -- {:?}",
                        //     i,
                        //     self.register,
                        //     i + 1,
                        //     value
                        // );
                        println!("SCHEDULE ADD: s:{:?} r:{:?}", i + 1, self.register);
                        pending_instruction = Some((i + 1, instruction.clone()));
                    }
                }
            } else {
                println!("{:?}: Outta instructions", i);
            }

            println!("AFTER: s:{:?} r:{:?}", i + 1, self.register);
        }
    }
}
