use std::fs;
use anyhow::Result;
use md5::{Md5, Digest};

pub fn execute() -> Result<()> {
    let input = fs::read_to_string("./inputs/4.txt")?;

    println!("# Day 4");
    println!("Part 1: {:?}", solve_part_1(&input));
    println!("Part 2: {:?}", solve_part_2(&input));

    Ok(())
}

fn solve_part_1(input: &str) -> usize {
    let mut answer = 0;

    loop {
        let input = format!("{}{}", input, answer);
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());

        let result = &hasher.finalize()[..];
        let result = hex::encode(result);
        let result = result.as_bytes();

        if &result[..5] == "00000".as_bytes() {
            return answer;
        } else {
            answer += 1;
        }
    }
}

fn solve_part_2(input: &str) -> usize {
    let mut answer = 0;

    loop {
        let input = format!("{}{}", input, answer);
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());

        let result = &hasher.finalize()[..];
        let result = hex::encode(result);
        let result = result.as_bytes();

        if &result[..6] == "000000".as_bytes() {
            return answer;
        } else {
            answer += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve_part_1("abcdef"), 609043);
        assert_eq!(solve_part_1("pqrstuv"), 1048970);
    }

}
