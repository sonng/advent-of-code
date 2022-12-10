use std::{collections::HashSet, fs};

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day9.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let results: Vec<Instruction> = input
        .split("\n")
        .flat_map(|l| Instruction::try_from(l))
        .collect();

    let mut visited = HashSet::new();
    let mut bridge = RopeBridge::new();

    visited.insert(bridge.tail);
    for instruction in results {
        for _ in 0..instruction.times() {
            bridge.actual_move_head(instruction.direction());
            visited.insert(bridge.tail);
        }
    }

    println!("Day 9-1: {:?}", visited.len());
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let results: Vec<Instruction> = input
        .split("\n")
        .flat_map(|l| Instruction::try_from(l))
        .collect();

    let mut visited = HashSet::new();
    let mut snake = Snake::new(10);

    visited.insert(snake.tail());
    for instruction in results {
        for _ in 0..instruction.times() {
            snake.move_head(instruction.direction());
            visited.insert(snake.tail());
        }
    }

    println!("Day 9-2: {:?}", visited.len());
    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Right(i64),
    Left(i64),
    Down(i64),
    Up(i64),
}

impl Instruction {
    fn times(&self) -> i64 {
        match self {
            Self::Right(v) => *v,
            Self::Left(v) => *v,
            Self::Down(v) => *v,
            Self::Up(v) => *v,
        }
    }

    fn direction(&self) -> Coord {
        match self {
            Self::Right(_) => Coord { x: -1, y: 0 },
            Self::Left(_) => Coord { x: 1, y: 0 },
            Self::Down(_) => Coord { x: 0, y: -1 },
            Self::Up(_) => Coord { x: 0, y: 1 },
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(' ').collect();

        if split.len() != 2 {
            return Err(Errors::ParseError("Invalid input length".into()));
        }

        let value: i64 = split[1].parse()?;

        match split[0] {
            "R" => Ok(Self::Right(value)),
            "U" => Ok(Self::Up(value)),
            "L" => Ok(Self::Left(value)),
            "D" => Ok(Self::Down(value)),
            _ => Err(Errors::ParseError("Invalid direction".into())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn origin() -> Self {
        Coord::new(0, 0)
    }

    fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    fn move_point(&mut self, direction: Coord) {
        self.x += direction.x;
        self.y += direction.y;
    }

    fn move_point_clone(&self, direction: Coord) -> Coord {
        let mut coord = self.clone();
        coord.move_point(direction);
        coord
    }

    fn follow_point(&mut self, other: &Coord) {
        if self.distance(other) == 0 || self.is_diagonal(other) || self.distance(other) == 1 {
            return;
        }

        // 4 -- 6
        if self.x == other.x {
            self.y += Coord::axis_direction(self.y, other.y);
        } else if self.y == other.y {
            self.x += Coord::axis_direction(self.x, other.x);
        } else {
            self.x += Coord::axis_direction(self.x, other.x);
            self.y += Coord::axis_direction(self.y, other.y);
        }
    }

    fn axis_direction(h: i64, t: i64) -> i64 {
        if h < t {
            1
        } else {
            -1
        }
    }

    fn distance(&self, other: &Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn is_diagonal(&self, other: &Coord) -> bool {
        other.move_point_clone(Coord { x: 1, y: 1 }) == *self
            || other.move_point_clone(Coord { x: 1, y: -1 }) == *self
            || other.move_point_clone(Coord { x: -1, y: 1 }) == *self
            || other.move_point_clone(Coord { x: -1, y: -1 }) == *self
    }
}

#[derive(Debug)]
struct Snake {
    body: Vec<Coord>,
}

impl Snake {
    fn new(size: usize) -> Self {
        Snake {
            body: vec![Coord::origin(); size],
        }
    }

    fn tail(&self) -> Coord {
        self.body.last().unwrap().clone()
    }

    fn move_head(&mut self, coord: Coord) {
        self.body[0].move_point(coord);

        for i in 1..self.body.len() {
            let prev = &self.body[i - 1].clone();
            self.body[i].follow_point(prev);
        }
    }
}

struct RopeBridge {
    head: Coord,
    tail: Coord,
}

impl RopeBridge {
    fn new_all_coords(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RopeBridge {
            head: Coord { x: x1, y: y1 },
            tail: Coord { x: x2, y: y2 },
        }
    }
    fn new() -> Self {
        Self::new_all_coords(0, 0, 0, 0)
    }

    fn actual_move_head(&mut self, coord: Coord) {
        self.head.move_point(coord);
        self.tail.follow_point(&self.head);
    }
}

#[cfg(test)]
mod tests {
    use super::Coord;

    #[test]
    fn test_move_up() {
        let head = Coord { x: 0, y: 2 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, 0);
        assert_eq!(tail.y, 1);
    }

    #[test]
    fn test_move_down() {
        let head = Coord { x: 0, y: -2 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, 0);
        assert_eq!(tail.y, -1);
    }

    #[test]
    fn test_move_left() {
        let head = Coord { x: 2, y: 0 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, 1);
        assert_eq!(tail.y, 0);
    }

    #[test]
    fn test_move_right() {
        let head = Coord { x: -2, y: 0 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, -1);
        assert_eq!(tail.y, 0);
    }

    #[test]
    fn test_move_up_right() {
        let head = Coord { x: 1, y: 2 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, 1);
        assert_eq!(tail.y, 1);
    }

    #[test]
    fn test_move_up_left() {
        let head = Coord { x: -1, y: 2 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, -1);
        assert_eq!(tail.y, 1);
    }

    #[test]
    fn test_move_down_left() {
        let head = Coord { x: -1, y: -2 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, -1);
        assert_eq!(tail.y, -1);
    }

    #[test]
    fn test_move_down_right() {
        let head = Coord { x: 1, y: -2 };
        let mut tail = Coord::origin();
        tail.follow_point(&head);

        assert_eq!(tail.x, 1);
        assert_eq!(tail.y, -1);
    }
}
