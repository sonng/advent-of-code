use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day12.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

struct Puzzle {
    start: Vec<Coord>,
    end: Coord,
    maze: Vec<Vec<u8>>,
}

impl TryFrom<&str> for Puzzle {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let maze: Vec<Vec<u8>> = value
            .split("\n")
            .map(|l| l.as_bytes().iter().map(|c| *c).collect())
            .collect();

        let mut start = vec![];
        let mut end = None;

        for y in 0..maze.len() {
            for x in 0..maze[y].len() {
                match maze[y][x] {
                    b'S' | b'a' => start.push(Coord::new(x as i64, y as i64)),
                    b'E' => end = Option::Some(Coord::new(x as i64, y as i64)),
                    _ => {}
                }
            }
        }

        let end = end.ok_or(Errors::ParseError("Could not find end".into()))?;

        Ok(Puzzle { start, end, maze })
    }
}

fn solve_part_1(input: &str) -> Result<()> {
    let puzzle = Puzzle::try_from(input)?;

    println!("Day 12-1: {:?}", puzzle.shortest_distance(puzzle.start[0]));
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let puzzle = Puzzle::try_from(input)?;

    let distances = puzzle
        .start
        .iter()
        .filter_map(|s| puzzle.shortest_distance(*s))
        .min();

    println!("Day 12-2: {:?}", distances.unwrap());
    Ok(())
}

impl Puzzle {
    fn shortest_distance(&self, start: Coord) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut steps = 0;

        let mut visited = HashSet::new();

        queue.push_back(start);

        while !queue.is_empty() {
            let to_deplete = queue.len();

            for _ in 0..to_deplete {
                let cur = queue.pop_front().unwrap();

                if !self.within(cur) || visited.contains(&cur) {
                    continue;
                }

                if self.maze[cur.y()][cur.x()] == b'E' {
                    return Some(steps);
                }

                visited.insert(cur);

                if self.can_move(cur, Coord::new(cur.x - 1, cur.y)) {
                    queue.push_back(Coord::new(cur.x - 1, cur.y));
                }
                if self.can_move(cur, Coord::new(cur.x + 1, cur.y)) {
                    queue.push_back(Coord::new(cur.x + 1, cur.y));
                }
                if self.can_move(cur, Coord::new(cur.x, cur.y - 1)) {
                    queue.push_back(Coord::new(cur.x, cur.y - 1));
                }
                if self.can_move(cur, Coord::new(cur.x, cur.y + 1)) {
                    queue.push_back(Coord::new(cur.x, cur.y + 1));
                }
            }

            steps += 1;
        }

        None
    }

    fn within(&self, coord: Coord) -> bool {
        coord.y >= 0
            && coord.y() < self.maze.len()
            && coord.x >= 0
            && coord.x() < self.maze[coord.y()].len()
    }

    fn can_move(&self, cur_position: Coord, intention: Coord) -> bool {
        if !self.within(intention) {
            return false;
        };

        let intent_c = self.maze[intention.y()][intention.x()];
        let cur_c = self.maze[cur_position.y()][cur_position.x()];

        if intent_c == b'S' {
            return false;
        }

        if cur_c == b'S' {
            return true;
        }

        if intent_c as i64 - cur_c as i64 > 1 {
            return false;
        }

        if intent_c == b'E' && cur_c != b'z' {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Coord { x, y }
    }

    fn y(&self) -> usize {
        self.y as usize
    }

    fn x(&self) -> usize {
        self.x as usize
    }
}
