use std::{cmp::max, collections::HashSet, fs, iter::zip};

use anyhow::{Ok, Result};

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day8.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let input: Vec<&[u8]> = input.split('\n').map(|v| v.as_bytes()).collect();

    let mut processed_input = vec![];
    for x in 0..input.len() {
        let mut cols = vec![];
        for y in 0..input[x].len() {
            cols.push((Coord { x, y }, input[x][y]));
        }
        processed_input.push(cols);
    }

    let results = visible_trees(&processed_input);

    println!("Day 8-1: {}", results);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let input: Vec<&[u8]> = input.split('\n').map(|v| v.as_bytes()).collect();

    let mut total_scores = vec![];
    for y in 0..input.len() {
        let mut scores = vec![];
        for x in 0..input[y].len() {
            let item = input[y][x];
            let mut score = 1;

            // right
            let mut tmp_score = 0;
            if (x + 1) <= input[y].len() {
                for i in &input[y][(x + 1)..input[y].len()] {
                    tmp_score += 1;

                    if item <= *i {
                        break;
                    }
                }
                println!("{:?},{:?} - {:?}", x, y, tmp_score);
            }

            score *= tmp_score;

            // left
            let mut tmp_score = 0;

            if x > 0 {
                for i in input[y][0..x].iter().rev() {
                    // println!("{:?} vs {:?}", item, *i);
                    tmp_score += 1;

                    if item <= *i {
                        break;
                    }
                }
            }

            println!("{:?},{:?} - {:?}", x, y, tmp_score);
            score *= tmp_score;
            // up
            let mut tmp_score = 0;

            if y > 0 {
                for i in (0..y).rev() {
                    let t = input[i][x];
                    tmp_score += 1;

                    if item <= t {
                        break;
                    }
                }
            }

            println!("{:?},{:?} - {:?}", x, y, tmp_score);
            score *= tmp_score;
            // down
            let mut tmp_score = 0;

            if y + 1 <= input.len() {
                for i in (y + 1)..input.len() {
                    let t = input[i][x];
                    tmp_score += 1;

                    if item <= t {
                        break;
                    }
                }
            }

            println!("{:?},{:?} - {:?}", x, y, tmp_score);
            score *= tmp_score;

            scores.push(score);
        }

        total_scores.push(scores);
    }
    println!("{:?}", total_scores);

    let results = total_scores
        .iter()
        .map(|r| r.iter().fold(0, |acc, item| -> i32 { max(acc, *item) }))
        .fold(0, |acc, item| max(acc, item));

    println!("Day 8-2: {:?}", results);
    Ok(())
}

fn visible_trees(input: &Vec<Vec<(Coord, u8)>>) -> usize {
    // Left to right
    let vis_left = input.iter().map(|r| visible_num(r));
    // Check right to left
    let vis_right = input.iter().map(|r| {
        let s: Vec<(Coord, u8)> = r.iter().map(|(c, v)| (*c, *v)).rev().collect();
        visible_num(&s)
    });
    let mut horizontal =
        zip(vis_left, vis_right).fold(HashSet::<Coord>::new(), |mut acc, (mut left, right)| {
            left.extend(right);
            acc.extend(left);
            acc
        });

    let rotated = rotate(input);
    // Check up and down
    let vis_up = rotated.iter().map(|r| visible_num(r));
    // Check down and up
    let vis_down = rotated.iter().map(|r| {
        let s: Vec<(Coord, u8)> = r.iter().map(|(c, v)| (*c, *v)).rev().collect();
        visible_num(&s)
    });
    let vertical =
        zip(vis_up, vis_down).fold(HashSet::<Coord>::new(), |mut acc, (mut left, right)| {
            left.extend(right);
            acc.extend(left);
            acc
        });

    horizontal.extend(vertical);
    horizontal.len()
}

fn rotate(input: &Vec<Vec<(Coord, u8)>>) -> Vec<Vec<(Coord, u8)>> {
    let mut results = vec![];

    for i in (0..input[0].len()).into_iter() {
        results.push(input.iter().map(|r| r[i]).collect());
    }

    results
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn visible_num<'a>(input: &Vec<(Coord, u8)>) -> HashSet<Coord> {
    let mut visible = HashSet::new();
    let mut highest_seen = b'0' - 1;

    for i in input {
        if i.1 > highest_seen {
            visible.insert(i.0);
            highest_seen = i.1;
        }
    }

    visible
}
