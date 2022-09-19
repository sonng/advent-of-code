use std::collections::HashSet;
use std::fs;
use anyhow::{bail, Result};

pub fn execute() -> Result<()> {
    let input = fs::read("./inputs/3.txt")?;
    let input = &input[..];

    println!("# Day 3");
    println!("Part 1: {:?}", solve_part_1(input)?);
    println!("Part 2: {:?}", solve_part_2(input)?);

    Ok(())
}


fn solve_part_1(input: &[u8]) -> Result<usize> {
    let mut map = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    map.insert((x, y));

    for i in input {
        match i {
            b'^' => y -= 1,
            b'>' => x += 1,
            b'<' => x -= 1,
            b'v' => y += 1,
            _ => bail!("Invalid move: {:?}", i)
        }

        map.insert((x, y));
    }

    Ok(map.len())
}

fn solve_part_2(input: &[u8]) -> Result<usize> {
    let mut map = HashSet::new();

    let mut coords = [(0, 0), (0, 0)];
    map.insert((0, 0));

    for (idx, i) in input.iter().enumerate() {
        let idx = idx % 2;
        match i {
            b'^' => coords[idx].1 -= 1,
            b'>' => coords[idx].0 += 1,
            b'<' => coords[idx].0 -= 1,
            b'v' => coords[idx].1 += 1,
            _ => bail!("Invalid move: {:?}", i)
        }

        map.insert((coords[idx].0, coords[idx].1));
    }

    Ok(map.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve_part_1(">".as_bytes()).unwrap(), 2);
        assert_eq!(solve_part_1("^>v<".as_bytes()).unwrap(), 4);
        assert_eq!(solve_part_1("^v^v^v^v^v".as_bytes()).unwrap(), 2);

        assert_eq!(solve_part_2("^v".as_bytes()).unwrap(), 3);
        assert_eq!(solve_part_2("^>v<".as_bytes()).unwrap(), 3);
        assert_eq!(solve_part_2("^v^v^v^v^v".as_bytes()).unwrap(), 11);

    }
}

