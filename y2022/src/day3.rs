use std::{collections::HashSet, fs};

pub fn exec() -> anyhow::Result<()> {
    let input = fs::read_to_string("./inputs/day3.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> anyhow::Result<()> {
    let answer: u64 = input
        .split('\n')
        .map(|c| c.as_bytes())
        .map(common_in_each_half)
        .map(convert_to_points)
        .sum();
    println!("Day 3-1: {:?}", answer);
    Ok(())
}

fn convert_to_points(input: u8) -> u64 {
    if input >= 65 && input <= 90 {
        return (input - 38) as u64;
    }

    return (input - 96) as u64;
}

fn common_in_each_half(input: &[u8]) -> u8 {
    let right_start = input.len() / 2;

    common(&input[0..right_start], &input[right_start..])
}

fn common(left: &[u8], right: &[u8]) -> u8 {
    let mut seen = HashSet::new();
    for c in left.iter() {
        seen.insert(*c);
    }

    for c in right.iter() {
        if seen.contains(c) {
            return *c;
        }
    }

    return 0;
}

fn commons(left: &[u8], right: &[u8]) -> HashSet<u8> {
    let mut seen = HashSet::new();
    for c in left.iter() {
        seen.insert(*c);
    }

    let mut common = HashSet::new();
    for c in right.iter() {
        if seen.contains(c) {
            common.insert(*c);
        }
    }

    common
}

fn solve_part_2(input: &str) -> anyhow::Result<()> {
    let answer = input.split('\n').collect::<Vec<&str>>();
    let answer: u64 = answer
        .chunks_exact(3)
        .map(|c| {
            let left = commons(c[0].as_bytes(), c[1].as_bytes());
            let right = commons(c[1].as_bytes(), c[2].as_bytes());

            for c in right.iter() {
                if left.contains(c) {
                    return *c;
                }
            }

            0
        })
        .map(convert_to_points)
        .sum();

    println!("Day 3-2: {:?}", answer);
    Ok(())
}
