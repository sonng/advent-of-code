use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    rc::Rc,
};

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

    let monkeys: Vec<Rc<RefCell<Monkey>>> = monkeys
        .iter()
        .map(|m| Rc::new(RefCell::new(m.clone())))
        .collect();
    let mut monkey_mapping = HashMap::<String, Rc<RefCell<Monkey>>>::new();

    for m in &monkeys {
        monkey_mapping.insert(m.borrow().name.to_owned(), Rc::clone(m));
    }

    let mut inspect_count = HashMap::new();

    for _ in 0..20 {
        play_round(&monkeys, &mut monkey_mapping, &mut inspect_count, 3);
    }

    println!("Day 11-1: {:?}", inspect_count);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .filter_map(|c| Monkey::try_from(c).ok())
        .collect();

    let monkeys: Vec<Rc<RefCell<Monkey>>> = monkeys
        .iter()
        .map(|m| Rc::new(RefCell::new(m.clone())))
        .collect();
    let mut monkey_mapping = HashMap::<String, Rc<RefCell<Monkey>>>::new();

    for m in &monkeys {
        monkey_mapping.insert(m.borrow().name.to_owned(), Rc::clone(m));
    }

    let mut inspect_count = HashMap::new();
    let relief = monkeys
        .iter()
        .map(|m| m.borrow().divisor())
        .fold(1, |acc, i| acc * i);

    for _ in 0..10000 {
        play_round(&monkeys, &mut monkey_mapping, &mut inspect_count, relief);
    }

    println!("Day 11-2: {:?}", inspect_count);
    Ok(())
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
struct Test {
    divisible: usize,
    true_case: usize,
    false_case: usize,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Squared,
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
            return Ok(Operation::Squared);
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

impl Operation {
    fn act(&self, value: usize) -> usize {
        match self {
            Self::Add(i) => value + i,
            Self::Multiply(i) => value * i,
            Self::Squared => value * value,
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
            .collect::<VecDeque<usize>>();

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

fn play_round(
    monkeys: &Vec<Rc<RefCell<Monkey>>>,
    mapping: &mut HashMap<String, Rc<RefCell<Monkey>>>,
    inspect_count: &mut HashMap<String, usize>,
    relief: usize,
) {
    for monkey in monkeys {
        play_monkey(Rc::clone(&monkey), mapping, inspect_count, relief);
    }
}

fn play_monkey(
    monkey: Rc<RefCell<Monkey>>,
    mapping: &mut HashMap<String, Rc<RefCell<Monkey>>>,
    inspect_count: &mut HashMap<String, usize>,
    relief: usize,
) {
    let operation = monkey.borrow().operation;
    let true_case = format!("{}", monkey.borrow().test.true_case);
    let false_case = format!("{}", monkey.borrow().test.false_case);
    let divisible = monkey.borrow().test.divisible;
    let name = monkey.borrow().name.clone();

    while let Some(item) = monkey.borrow_mut().items.pop_front() {
        inspect_count
            .entry(name.clone())
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);

        // println!("Monkey inspects an item with worry level of {}", item);
        let item = operation.act(item);
        // println!("Worry level is {:?} to {}", operation, item);
        let item = item % relief;
        // println!(
        //     "Monkey gets bored with item. Worry level is divided by 3 to {}",
        //     item
        // );

        let toss_to = if item % divisible == 0 {
            // println!("Current worry level is divisible by {}", divisible);
            true_case.clone()
        } else {
            // println!("Current worry level is not divisible by {}", divisible);
            false_case.clone()
        };

        // println!(
        //     "Item with worry level {} is thrown to monkey {}",
        //     item, toss_to
        // );
        mapping.entry(toss_to).and_modify(|m| {
            m.borrow_mut().items.push_back(item);
        });
    }
}

impl Monkey {
    fn divisor(&self) -> usize {
        self.test.divisible
    }
}
