use std::{cell::RefCell, fs, rc::Rc};

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

    cpu.exec(
        instructions.into_iter().rev().collect(),
        220,
        |stage| match stage {
            Stage::During(step, value) => {
                let step = step + 1;
                match step {
                    20 | 60 | 100 | 140 | 180 | 220 => results.push(step as i64 * value),
                    _ => {}
                };
            }
            _ => {}
        },
    );
    println!("Results: {:?}", results);
    println!("Day 10-1: {:?}", results.iter().sum::<i64>());
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let instructions: Vec<Instruction> = input
        .split("\n")
        .filter_map(|c| Instruction::try_from(c).ok())
        .collect();

    let mut cpu = CPU::new();
    let crt = Rc::new(RefCell::new(CRT::new(40, 6)));

    let crt_1 = Rc::clone(&crt);
    let crt_2 = Rc::clone(&crt);

    cpu.exec(instructions.into_iter().rev().collect(), 40 * 6, |stage| {
        match stage {
            Stage::During(step, value) => {
                crt_1.borrow_mut().plot(step as i64);
                crt_2.borrow_mut().move_sprite(value);
            }
            _ => {}
        }
        // println!(
        //     "DURING: s:{:?} r: {:?} = {:?}",
        //     step + 1,
        //     value,
        //     "" // (step + 1) as i64 * value
        // );
    });

    crt.borrow().print();
    println!("Day 10-2: {:?}", "");
    Ok(())
}

#[derive(Clone, Copy)]
enum Instruction {
    Addx(i64),
    Noop,
}

enum Stage {
    Start(usize, i64),
    During(usize, i64),
    EndOfCycle(usize, i64),
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
}

impl CPU {
    fn new() -> Self {
        CPU { register: 1 }
    }

    fn peek(&self) -> i64 {
        self.register
    }

    fn exec<F>(&mut self, instructions: Vec<Instruction>, max_steps: usize, mut callback: F)
    where
        F: FnMut(Stage),
    {
        let mut instructions: Vec<Instruction> = instructions.clone();
        let mut pending_instruction: Option<(usize, Instruction)> = None;
        for i in 0..max_steps {
            callback(Stage::Start(i, self.register));

            if pending_instruction.is_none() {
                print!("Start cycle {}: begin executing ", i);
                if let Some(instruction) = instructions.pop() {
                    match instruction {
                        Instruction::Noop => {
                            println!("noop");
                        }
                        Instruction::Addx(value) => {
                            println!("addx {}", value);
                            pending_instruction = Some((i + 1, instruction.clone()));
                        }
                    }
                } else {
                    println!("no instructions left!");
                }
                println!("During cycle {}", i);
                callback(Stage::During(i, self.register));
                println!("End of cycle {}: finish executing", i);
            } else if let Some((when, instruction)) = pending_instruction {
                println!("During cycle {}", i);
                callback(Stage::During(i, self.register));
                print!("End of cycle {}: finish executing ", i);
                if i == when {
                    match instruction {
                        Instruction::Noop => {
                            println!("Should never get here!");
                        }
                        Instruction::Addx(value) => {
                            self.register += value;
                            println!("addx {} (Register X is now {}", value, self.register);
                            // println!(
                            //     "{:?}-{:?}: Executed {:?} -- {:?} -> {:?}",
                            //     i, self.register, when, value, self.register
                            // );
                        }
                    }
                    pending_instruction = None;
                } else {
                    println!("noop");
                }
            } else {
                println!("{:?}: Outta instructions", i);
                break;
            }
            callback(Stage::EndOfCycle(i, self.register));
        }
    }
}

struct CRT {
    width: usize,
    height: usize,
    display: Vec<u8>,
    position: i64,
}

impl CRT {
    fn new(width: usize, height: usize) -> Self {
        CRT {
            width,
            height,
            display: vec![],
            position: 1,
        }
    }

    fn move_sprite(&mut self, position: i64) {
        self.position = position;
    }

    fn cursor_on_position(&self, step: i64) -> bool {
        step >= self.position - 1 && step <= self.position + 1
    }

    fn plot(&mut self, step: i64) {
        if step >= (self.width * self.height) as i64 {
            println!("Returning earlier");
            return;
        }

        let step = step as i64;
        if self.cursor_on_position(step) {
            println!("Drawing #: {:?}", self.position);

            self.display.push(b'#');
        } else {
            println!("Drawing .: {:?}", self.position);
            self.display.push(b'.');
        }
    }

    fn print(&self) {
        for c in self.display.iter().map(|c| *c as char).enumerate() {
            if c.0 % self.width == 0 && c.0 != 0 {
                println!("");
            }
            print!("{}", c.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CRT;

    #[test]
    fn test_cur_position() {
        let mut crt = CRT::new(40, 6);

        crt.move_sprite(1);
        assert!(!crt.cursor_on_position(-1));
        assert!(crt.cursor_on_position(0));
        assert!(crt.cursor_on_position(1));
        assert!(crt.cursor_on_position(2));
        assert!(!crt.cursor_on_position(3));

        crt.move_sprite(45);
        assert!(!crt.cursor_on_position(43));
        assert!(crt.cursor_on_position(44));
        assert!(crt.cursor_on_position(45));
        assert!(crt.cursor_on_position(46));
        assert!(!crt.cursor_on_position(47));
    }
}
