use std::fs;

pub fn execute() -> anyhow::Result<()> {
    println!("# Day 1");
    let file = fs::read("./inputs/1.txt")?;
    let file = &file[..];
    println!("Part 1: {:?}", solve_part_1(file));
    println!("Part 2: {:?}", solve_part_2(file));

    Ok(())
}

fn solve_part_1(input: &[u8]) -> i64 {
    input.iter().map(convert).sum()
}

fn convert(value: &u8) -> i64 {
    match value {
        b'(' => 1,
        b')' => -1,
        _ => 0
    }
}

fn solve_part_2(input: &[u8]) -> usize {
    let mut floor = 0;
    for (idx, i) in input.iter().enumerate() {
        let value = convert(i);
        floor += value;

        if floor < 0 {
            return idx + 1;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1_examples() {
        assert_eq!(solve_part_1("(())".as_bytes()), 0);
        assert_eq!(solve_part_1("()()".as_bytes()), 0);
        assert_eq!(solve_part_1("(((".as_bytes()), 3);
        assert_eq!(solve_part_1("(()(()(".as_bytes()), 3);
        assert_eq!(solve_part_1("))(((((".as_bytes()), 3);
        assert_eq!(solve_part_1("())".as_bytes()), -1);
        assert_eq!(solve_part_1("))(".as_bytes()), -1);
        assert_eq!(solve_part_1(")))".as_bytes()), -3);
        assert_eq!(solve_part_1(")())())".as_bytes()), -3);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(solve_part_2(")".as_bytes()), 1);
        assert_eq!(solve_part_2("()())".as_bytes()), 5);
    }
}