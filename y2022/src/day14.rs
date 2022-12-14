use std::{cmp::max, fs};

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day14.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let lines: Vec<Line> = input
        .split('\n')
        .filter_map(|l| Coords::try_from(l).ok())
        .flat_map(map_coords_to_lines)
        .collect();

    let mut cave = Cave::from(lines);

    let mut results = 0;
    while cave.drop_sand(Coord::new(499, 0)) {
        results += 1;
    }

    println!("{}", cave.to_string());

    println!("Day 14-1: {}", results);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 14-2: {:?}", "");
    Ok(())
}

struct Cave {
    map: Vec<Vec<Unit>>,
}

impl From<Vec<Line>> for Cave {
    fn from(lines: Vec<Line>) -> Self {
        let max_x = lines
            .iter()
            .fold(isize::MIN, |prev, line| max(prev, line.max_x()))
            + 1;
        let max_y = lines
            .iter()
            .fold(isize::MIN, |prev, line| max(prev, line.max_y()))
            + 1;

        let mut map = vec![vec![Unit::Air; max_x as usize]; max_y as usize];

        for line in lines.iter() {
            for coord in line.iter() {
                map[coord.y as usize][coord.x as usize] = Unit::Rock;
            }
        }

        Cave { map }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }

    fn add(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn direction_towards(&self, other: &Coord) -> Coord {
        Coord {
            x: if self.x == other.x {
                0
            } else {
                if self.x < other.x {
                    1
                } else {
                    -1
                }
            },
            y: if self.y == other.y {
                0
            } else {
                if self.y < other.y {
                    1
                } else {
                    -1
                }
            },
        }
    }
}

impl TryFrom<&str> for Coord {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: Vec<isize> = value
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        if value.len() != 2 {
            return Err(Errors::ParseError("Issues parsing coords".into()));
        }

        Ok(Coord::new(value[0] - 1, value[1] - 1))
    }
}

struct Coords(Vec<Coord>);

impl TryFrom<&str> for Coords {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coords: Vec<Coord> = value
            .split("->")
            .filter_map(|c| Coord::try_from(c).ok())
            .collect();

        Ok(Self(coords))
    }
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

struct LineIterator {
    start: Coord,
    end: Coord,
    direction: Coord,
}

impl Iterator for LineIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.direction == Coord::new(0, 0) {
            return None;
        }

        if self.start == self.end {
            self.direction = Coord::new(0, 0);
            return Some(self.end);
        }

        let result = self.start;

        self.start.x += self.direction.x;
        self.start.y += self.direction.y;

        Some(result)
    }
}

impl Line {
    fn iter(&self) -> LineIterator {
        LineIterator {
            start: self.start,
            end: self.end,
            direction: self.start.direction_towards(&self.end),
        }
    }

    fn max_x(&self) -> isize {
        max(self.start.x, self.end.x)
    }

    fn max_y(&self) -> isize {
        max(self.start.y, self.end.y)
    }
}

fn map_coords_to_lines(coords: Coords) -> Vec<Line> {
    coords.0[..]
        .windows(2)
        .map(|coords| Line {
            start: coords[0],
            end: coords[1],
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Unit {
    Air,
    Rock,
    Sand,
}

impl Unit {
    fn to_char(&self) -> char {
        match self {
            Self::Air => '.',
            Self::Rock => '#',
            Self::Sand => 'o',
        }
    }
}

impl ToString for Cave {
    fn to_string(&self) -> String {
        let mut results = String::new();

        for line in self.map.iter() {
            for col in line.iter() {
                results.push(col.to_char());
            }
            results.push('\n');
        }

        results
    }
}

impl Cave {
    // Returns true if it rests, returns false if it doesn't
    fn drop_sand(&mut self, from: Coord) -> bool {
        let mut sand = from;
        let down = Coord::new(0, 1);
        let down_left = Coord::new(-1, 1);
        let down_right = Coord::new(1, 1);

        loop {
            let new_pos = sand.add(&down);
            if !self.within_map(new_pos) {
                return false;
            }
            if !self.is_taken(new_pos) {
                sand = new_pos;
                continue;
            }

            let new_pos = sand.add(&down_left);
            if !self.within_map(new_pos) {
                return false;
            }
            if !self.is_taken(new_pos) {
                sand = new_pos;
                continue;
            }

            let new_pos = sand.add(&down_right);
            if !self.within_map(new_pos) {
                return false;
            }
            if !self.is_taken(new_pos) {
                sand = new_pos;
                continue;
            }

            self.map[sand.y as usize][sand.x as usize] = Unit::Sand;

            break;
        }

        return true;
    }

    fn within_map(&self, coord: Coord) -> bool {
        if coord.x < 0
            || coord.x >= self.map[0].len() as isize
            || coord.y >= self.map.len() as isize
        {
            return false;
        }

        true
    }

    fn is_taken(&self, coord: Coord) -> bool {
        self.map[coord.y as usize][coord.x as usize] != Unit::Air
    }
}
