use std::fs;

use anyhow::Result;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day5.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

#[derive(Debug)]
struct Puzzle {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

impl TryFrom<&str> for Puzzle {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split("\n\n").collect();

        if split.len() != 2 {
            return Err(());
        }

        let stacks: Vec<Vec<char>> = Puzzle::parse_stacks(split[0]);

        let instructions: Vec<Instruction> = split[1]
            .split('\n')
            .filter_map(|s| Instruction::try_from(s).ok())
            .collect();

        Ok(Puzzle {
            stacks,
            instructions,
        })
    }
}

impl Puzzle {
    fn parse_stacks(input: &str) -> Vec<Vec<char>> {
        let input: Vec<&str> = input.split('\n').collect();
        let mut results: Vec<Vec<char>> = vec![];

        let mut idx = 1;

        while idx <= input.last().unwrap().len() {
            let mut stack = vec![];
            for n in (0..(input.len() - 1)).rev() {
                if input[n].as_bytes()[idx] == b' ' {
                    continue;
                }
                stack.push(input[n].as_bytes()[idx] as char);
            }
            results.push(stack);
            idx = idx + 4;
        }

        results
    }

    fn play(&mut self) {
        for instruction in &self.instructions {
            for _ in 0..instruction.stack {
                if let Some(item) = self.stacks[instruction.from - 1].pop() {
                    self.stacks[instruction.to - 1].push(item);
                }
            }
        }
    }

    fn play_weird(&mut self) {
        for instruction in &self.instructions {
            let items: Vec<char> = (0..instruction.stack)
                .into_iter()
                .filter_map(|_| self.stacks[instruction.from - 1].pop())
                .collect();

            items
                .iter()
                .rev()
                .for_each(|c| self.stacks[instruction.to - 1].push(*c));
        }
    }

    fn top(&self) -> String {
        self.stacks.iter().filter_map(|s| s.last()).collect()
    }
}

#[derive(Debug)]
struct Instruction {
    stack: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<usize> = value.split(' ').filter_map(|s| s.parse().ok()).collect();

        if split.len() != 3 {
            return Err(());
        }

        Ok(Instruction {
            stack: split[0],
            from: split[1],
            to: split[2],
        })
    }
}

fn solve_part_1(input: &str) -> Result<()> {
    let mut puzzle = Puzzle::try_from(input).unwrap();
    puzzle.play();
    println!("Day 5-1: {:?}", puzzle.top());
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let mut puzzle = Puzzle::try_from(input).unwrap();
    puzzle.play_weird();
    println!("Day 5-2: {:?}", puzzle.top());
    Ok(())
}
