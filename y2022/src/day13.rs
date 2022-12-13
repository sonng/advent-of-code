use std::fs;

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day13.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let comparisons: Vec<Comparison> = input
        .split("\n\n")
        .filter_map(|ls| Comparison::try_from(ls).ok())
        .collect();

    let valid: usize = comparisons
        .iter()
        .enumerate()
        .filter(|(idx, c)| c.left <= c.right)
        .map(|(idx, _)| idx + 1)
        .sum();

    println!("Day 13-1: {:?}", valid);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let packets: Vec<&str> = input
        .split("\n\n")
        .flat_map(|c| c.split('\n').collect::<Vec<&str>>())
        .collect();

    let mut packets: Vec<Packet> = packets
        .iter()
        .filter_map(|s| Packet::try_from(*s).ok())
        .collect();

    let first_divider = Packet::divider(2);
    let second_divider = Packet::divider(6);

    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    packets.sort();

    let first_index = packets.binary_search(&first_divider).unwrap();
    let second_index = packets.binary_search(&second_divider).unwrap();

    println!("Day 13-2: {:?}", (first_index + 1) * (second_index + 1));
    Ok(())
}

#[derive(Debug)]
struct Comparison {
    left: Packet,
    right: Packet,
}

impl TryFrom<&str> for Comparison {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split('\n').collect();

        if split.len() != 2 {
            return Err(Errors::ParseError("Invalid comparison".into()));
        }

        let left = Packet::try_from(split[0])?;
        let right = Packet::try_from(split[1])?;

        Ok(Comparison { left, right })
    }
}

#[derive(Debug, Clone)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn new(value: usize) -> Self {
        Packet::Value(value)
    }

    fn new_list() -> Self {
        Packet::List(vec![])
    }

    fn divider(value: usize) -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Value(value)])])
    }

    fn insert_list(&mut self, other: Packet) {
        match self {
            Self::Value(_) => {}
            Self::List(list) => list.push(other),
        }
    }

    fn insert(&mut self, value: usize) {
        match self {
            Self::Value(_) => {}
            Self::List(list) => list.push(Self::new(value)),
        }
    }
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Self::Value(v) => format!("{}", *v),
            Self::List(l) => format!(
                "[{}]",
                l.iter()
                    .map(Packet::to_string)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

impl TryFrom<&str> for Packet {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stack = vec![];

        let value = value.as_bytes();
        let mut detect_value = None;
        for i in 0..value.len() {
            match value[i] {
                b'[' => stack.push(Self::new_list()),
                b']' | b',' => {
                    // Check if there is a value, insert into array
                    if let Some(idx) = detect_value {
                        let v = std::str::from_utf8(&value[idx..i])?.parse::<usize>()?;
                        stack.last_mut().unwrap().insert(v);
                    }

                    detect_value = None;
                    if value[i] == b']' {
                        if stack.len() > 1 {
                            let l = stack.pop().unwrap();
                            stack.last_mut().unwrap().insert_list(l);
                        }
                    }
                }
                _ => {
                    if detect_value.is_none() {
                        detect_value = Some(i);
                    }
                }
            }
        }

        Ok(stack[0].clone())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Value(l), Self::Value(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => l.cmp(r),
            (Self::Value(l), Self::List(r)) => {
                let l = vec![Packet::new(*l)];
                l.cmp(r)
            }
            (Self::List(l), Self::Value(r)) => {
                let r = vec![Packet::new(*r)];
                l.cmp(&r)
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l), Self::Value(r)) => l == r,
            (Self::List(l), Self::List(r)) => l == r,
            (Self::Value(l), Self::List(r)) => {
                let l = vec![Packet::new(*l)];
                &l == r
            }
            (Self::List(l), Self::Value(r)) => {
                let r = vec![Packet::new(*r)];
                l == &r
            }
        }
    }
}

impl Eq for Packet {}
