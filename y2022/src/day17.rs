use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;

use crate::{Coord, Errors};

pub fn exec() -> Result<()> {
    let input_example = fs::read_to_string("./inputs/day17_example.txt")?;
    let input = fs::read_to_string("./inputs/day17.txt")?;
    solve_part_1(&input_example)?;
    solve_part_1(&input)?;
    solve_part_2(&input_example)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let highest_point = solve(input, 2022);

    println!("Day 17-1: {}", highest_point);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let highest_point = solve(input, 1000000000000);

    println!("Day 17-2: {}", highest_point);
    Ok(())
}

fn solve(input: &str, total_shapes: usize) -> isize {
    let directions: Vec<Direction> = input
        .as_bytes()
        .iter()
        .filter_map(|s| Direction::try_from(*s).ok())
        .collect();

    let mut terrian = Terrian::new_with_coords(vec![
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
        Coord::new(4, 0),
        Coord::new(5, 0),
        Coord::new(6, 0),
    ]);

    let binding = get_shapes();
    let mut shapes = binding.iter().cycle();
    let mut directions = directions.iter().cycle();

    run(&mut terrian, &mut shapes, &mut directions, 0, total_shapes)
}

struct Terrian(HashMap<isize, HashSet<Coord>>);

impl Terrian {
    fn new() -> Self {
        Terrian(HashMap::new())
    }

    fn new_with_coords<I>(coords: I) -> Self
    where
        I: IntoIterator<Item = Coord>,
    {
        let mut terrian = Self::new();

        terrian.lock_in(coords);

        terrian
    }

    fn is_valid_move(&self, moves: &HashSet<Coord>) -> bool {
        if moves.iter().any(|c| c.x < 0 || c.x >= BOUNDS) {
            return false;
        }

        !moves
            .iter()
            .any(|c| self.0.get(&c.y).map_or(false, |set| set.contains(&c)))
    }

    fn lock_in<I>(&mut self, moves: I)
    where
        I: IntoIterator<Item = Coord>,
    {
        moves.into_iter().for_each(|c| {
            self.0
                .entry(c.y)
                .and_modify(|h| {
                    h.insert(c);
                })
                .or_insert(HashSet::from_iter(vec![c.clone()]));
        });
    }

    fn difference(&self, coords: &HashSet<Coord>) -> Vec<Coord> {
        coords
            .iter()
            .filter_map(|c| {
                self.0
                    .get(&c.y)
                    .filter(|h| !h.contains(&c))
                    .and_then(|_| Some(c))
            })
            .map(|c| c.clone())
            .collect()
    }

    fn find_and_normalize(&self, row: isize, normalize_to: isize) -> HashSet<Coord> {
        self.0
            .get(&row)
            .map(|h| h.iter().map(|c| c.normalize_y_to(normalize_to)).collect())
            .unwrap_or(HashSet::new())
    }
}

fn run<'a, 'b>(
    terrian: &mut Terrian,
    shapes: &mut impl Iterator<Item = &'b Shape>,
    directions: &mut impl Iterator<Item = &'a Direction>,
    highest_point: isize,
    total_shapes: usize,
) -> isize {
    let mut highest_point = highest_point;
    let mut taken = 0;

    while let Some(shape) = shapes.next() {
        highest_point = max(
            highest_point,
            try_move(terrian, &shape, directions, highest_point),
        );

        taken += 1;
        if taken == total_shapes {
            break;
        }
    }

    highest_point
}

#[derive(Debug, Clone)]
struct Shape(HashSet<Coord>);

fn get_shapes() -> Vec<Shape> {
    vec![
        // ---
        Shape(HashSet::from_iter(vec![
            Coord::new(-1, 0),
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
        ])),
        // +
        Shape(HashSet::from_iter(vec![
            Coord::new(0, 0),
            Coord::new(0, -1),
            Coord::new(0, -2),
            Coord::new(-1, -1),
            Coord::new(1, -1),
        ])),
        // L
        Shape(HashSet::from_iter(vec![
            Coord::new(1, 0),
            Coord::new(1, -1),
            Coord::new(1, -2),
            Coord::new(0, -2),
            Coord::new(-1, -2),
        ])),
        // l
        Shape(HashSet::from_iter(vec![
            Coord::new(-1, 0),
            Coord::new(-1, -1),
            Coord::new(-1, -2),
            Coord::new(-1, -3),
        ])),
        // o
        Shape(HashSet::from_iter(vec![
            Coord::new(0, 0),
            Coord::new(0, -1),
            Coord::new(-1, 0),
            Coord::new(-1, -1),
        ])),
    ]
}

impl Shape {
    fn transform_by(&self, coord: Coord) -> HashSet<Coord> {
        self.0.iter().map(|c| c.adjust(&coord)).collect()
    }

    fn transform(&mut self, coord: Coord) {
        self.0 = self.transform_by(coord);
    }

    fn highest_point(&self) -> isize {
        self.0.iter().map(|c| c.y).max().unwrap_or(0)
    }

    fn lowest_point(&self) -> Coord {
        Coord {
            y: self.0.iter().map(|c| c.y).min().unwrap_or(0),
            x: self.0.iter().map(|c| c.x).min().unwrap_or(0),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

impl TryFrom<u8> for Direction {
    type Error = Errors;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Self::Left),
            b'>' => Ok(Self::Right),
            _ => Err(Errors::ParseError("Invalid character for direction".into())),
        }
    }
}

impl Direction {
    fn coord(&self) -> Coord {
        match self {
            Self::Left => Coord::new(-1, 0),
            Self::Right => Coord::new(1, 0),
            Self::Down => Coord::new(0, -1),
        }
    }
}

const BOUNDS: isize = 7;
fn try_move<'a>(
    terrian: &mut Terrian,
    shape: &Shape,
    directions: &mut impl Iterator<Item = &'a Direction>,
    highest_point: isize,
) -> isize {
    let mut shape = shape.clone();
    let lowest_pt = shape.lowest_point();

    // Put it on the starting position
    shape.transform(Coord::new(
        2 + lowest_pt.x.abs(),
        highest_point + 4 + lowest_pt.y.abs(),
    ));

    loop {
        let direction = directions.next().unwrap();
        let new_pos: HashSet<Coord> = shape.transform_by(direction.coord());

        // If the space is empty and isn't out of bounds
        if terrian.is_valid_move(&new_pos) {
            shape.transform(direction.coord());
        }

        // Gravity hits
        let new_pos: HashSet<Coord> = shape.transform_by(Direction::Down.coord());

        // If the space isn't empty it means it'll come to a stop
        if !terrian.is_valid_move(&new_pos) {
            break;
        }

        shape.transform(Direction::Down.coord());
    }

    let highest_point = shape.highest_point();

    terrian.lock_in(shape.0);

    highest_point
}

#[cfg(test)]
mod tests {
    use std::{cmp::max, collections::HashSet, fs};

    use crate::{
        day17::{try_move, Terrian},
        Coord,
    };

    use super::{get_shapes, Direction, Shape};

    #[test]
    fn test_positioning() {
        let mut shapes: Vec<Shape> = get_shapes();

        shapes
            .iter_mut()
            .for_each(|s| s.transform(Coord::new(3, 4)));

        assert_eq!(
            shapes[0].0.clone().into_iter().collect::<HashSet<Coord>>(),
            HashSet::from_iter(vec![
                Coord::new(2, 4),
                Coord::new(3, 4),
                Coord::new(4, 4),
                Coord::new(5, 4)
            ])
        );

        assert_eq!(
            shapes[1].0.clone().into_iter().collect::<HashSet<Coord>>(),
            HashSet::from_iter(vec![
                Coord::new(3, 3),
                Coord::new(3, 4),
                Coord::new(3, 2),
                Coord::new(4, 3),
                Coord::new(2, 3)
            ])
        );

        assert_eq!(
            shapes[2].0.clone().into_iter().collect::<HashSet<Coord>>(),
            HashSet::from_iter(vec![
                Coord::new(4, 4),
                Coord::new(4, 3),
                Coord::new(4, 2),
                Coord::new(3, 2),
                Coord::new(2, 2)
            ])
        );

        assert_eq!(
            shapes[3].0.clone().into_iter().collect::<HashSet<Coord>>(),
            HashSet::from_iter(vec![
                Coord::new(2, 4),
                Coord::new(2, 3),
                Coord::new(2, 2),
                Coord::new(2, 1),
            ])
        );

        assert_eq!(
            shapes[4].0.clone().into_iter().collect::<HashSet<Coord>>(),
            HashSet::from_iter(vec![
                Coord::new(2, 4),
                Coord::new(2, 3),
                Coord::new(3, 4),
                Coord::new(3, 3),
            ])
        );
    }

    #[test]
    fn test_objects_falling() {
        let input_example = fs::read_to_string("./inputs/day17_example.txt").unwrap();
        let directions: Vec<Direction> = input_example
            .as_bytes()
            .iter()
            .filter_map(|s| Direction::try_from(*s).ok())
            .collect();

        let expected = vec![
            Coord::new(0, 0),
            Coord::new(1, 0),
            Coord::new(2, 0),
            Coord::new(3, 0),
            Coord::new(4, 0),
            Coord::new(5, 0),
            Coord::new(6, 0),
        ];
        let mut terrian = Terrian::new_with_coords(expected.clone());

        let mut expected: HashSet<Coord> = HashSet::from_iter(expected);

        let binding = get_shapes();
        let mut shapes = binding.iter().cycle().take(5);
        let mut highest_point = 0;
        let mut directions = directions.iter().cycle();

        // Action ----
        let shape = shapes.next().unwrap();
        highest_point = max(
            highest_point,
            try_move(&mut terrian, shape, &mut directions, highest_point),
        );

        // Test ----
        expected.extend(vec![
            Coord::new(2, 1),
            Coord::new(3, 1),
            Coord::new(4, 1),
            Coord::new(5, 1),
        ]);

        let difference = terrian.difference(&expected);
        println!("Difference -: {:?}", difference);
        assert_eq!(difference.len(), 0);

        // Action +
        let shape = shapes.next().unwrap();
        highest_point = max(
            highest_point,
            try_move(&mut terrian, shape, &mut directions, highest_point),
        );

        //  Test +
        expected.extend(vec![
            Coord::new(3, 2),
            Coord::new(3, 3),
            Coord::new(4, 3),
            Coord::new(2, 3),
            Coord::new(3, 4),
        ]);

        let difference = terrian.difference(&expected);
        println!("Difference +: {:?}", difference);
        assert_eq!(difference.len(), 0);

        // Action L
        let shape = shapes.next().unwrap();
        highest_point = max(
            highest_point,
            try_move(&mut terrian, shape, &mut directions, highest_point),
        );

        // Test L
        expected.extend(vec![
            Coord::new(2, 4),
            Coord::new(1, 4),
            Coord::new(0, 4),
            Coord::new(2, 5),
            Coord::new(2, 6),
        ]);

        /*
          0 1 2 3 4 5 6
        | . . . . $ . . | 7
        | . . # . $ . . | 6
        | . . # $ $ . . | 5
        | # # # # . . . | 4
        | . . # # # . . | 3
        | . . . # . . . | 2
        | . . # # # # . | 1
        + - - - - - - - +

        */
        let difference = terrian.difference(&expected);
        println!("Difference L: {:?}", difference);
        assert_eq!(difference.len(), 0);

        // Action l
        let shape = shapes.next().unwrap();
        highest_point = max(
            highest_point,
            try_move(&mut terrian, shape, &mut directions, highest_point),
        );

        // l
        expected.extend(vec![
            Coord::new(4, 4),
            Coord::new(4, 5),
            Coord::new(4, 6),
            Coord::new(4, 7),
        ]);

        let difference = terrian.difference(&expected);
        println!("Difference l: {:?}", difference);
        assert_eq!(difference.len(), 0);

        // o
        let shape = shapes.next().unwrap();
        highest_point = max(
            highest_point,
            try_move(&mut terrian, shape, &mut directions, highest_point),
        );

        // o
        expected.extend(vec![
            Coord::new(4, 8),
            Coord::new(4, 9),
            Coord::new(5, 8),
            Coord::new(5, 9),
        ]);

        let difference = terrian.difference(&expected);
        println!("Difference o: {:?}", difference);
        assert_eq!(difference.len(), 0);
    }
}
