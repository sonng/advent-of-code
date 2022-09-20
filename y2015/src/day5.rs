use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::Result;

pub fn execute() -> Result<()> {
    let input = fs::read_to_string("./inputs/5.txt")?;
    let input = input.split('\n').collect::<Vec<&str>>();

    println!("# Day 5");
    println!("Part 1: {:?}", solve_part_1(&input));
    println!("Part 2: {:?}", solve_part_2(&input));

    Ok(())
}

fn is_nice_part_1(input: &str) -> bool {
    let mut vowel_count = 0;
    let mut twice_count = 0;

    let mut prev: Option<&u8> = None;
    for c in input.as_bytes() {
        if is_vowel(c) { vowel_count += 1 }
        if let Some(p) = prev {
            if p == c { twice_count += 1 }
        }

        if dead_set_bad(prev, c) { return false; }

        prev = Some(c);
    }

    vowel_count > 2 && twice_count > 0
}

fn dead_set_bad(prev: Option<&u8>, c: &u8) -> bool {
    match prev {
        Some(p) => {
            match p {
                b'a' if c == &b'b' => true,
                b'c' if c == &b'd' => true,
                b'p' if c == &b'q' => true,
                b'x' if c == &b'y' => true,
                _ => false
            }
        }
        _ => false,
    }
}

fn is_vowel(c: &u8) -> bool {
    match c {
        b'a' | b'e' | b'i' | b'o' | b'u' => true,
        _ => false
    }
}

fn is_nice_part_2(input: &str) -> bool {
    let input = input.as_bytes();

    let mut pairs: HashMap<(u8, u8), HashSet<usize>> = HashMap::new();

    let mut pair_appearance = false;
    let mut repeats_with_letter = false;

    let mut i = 1;
    while i < input.len() {
        let prev = input[i-1];
        let cur = input[i];

        let pair_entry = pairs.entry((prev, cur)).or_default();
        if !pair_entry.is_empty() &&
            pair_entry.difference(&HashSet::from([i - 1])).count() > 0 {
            pair_appearance = true;
        }
        pair_entry.insert(i);

        if i + 1 < input.len() && prev == input[i + 1] {
            repeats_with_letter = true;
        }

        i += 1;
    }

    pair_appearance && repeats_with_letter
}

fn solve_part_1(input: &Vec<&str>) -> usize {
    input.iter().filter(|c| is_nice_part_1(c)).count()
}

fn solve_part_2(input: &Vec<&str>) -> usize {
    input.iter().filter(|c| is_nice_part_2(c)).count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn examples() {
        assert_eq!(is_nice_part_1("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_part_1("aaa"), true);
        assert_eq!(is_nice_part_1("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_part_1("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_part_1("dvszwmarrgswjxmb"), false);

        assert_eq!(is_nice_part_2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_part_2("aaa"), false);
        assert_eq!(is_nice_part_2("aaaa"), true);
        assert_eq!(is_nice_part_2("xxyxx"), true);
        assert_eq!(is_nice_part_2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part_2("ieodomkazucvgmuy"), false);
    }
}

