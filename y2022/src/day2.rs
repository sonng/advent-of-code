use std::fs;

use Hand::*;
use RoundResult::*;

pub fn exec() -> anyhow::Result<()> {
    let input = fs::read_to_string("./inputs/day2.txt")?;

    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Hand {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Hand {
    fn score(&self) -> u32 {
        match &self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn fight(&self, other_hand: &Hand) -> u32 {
        match (self, other_hand) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => 0,
            (_, _) => 3,
        }
    }
}

struct Round {
    left: Hand,
    right: Hand,
}

impl TryFrom<&[u8]> for Round {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(());
        }

        let left = Hand::try_from(value[0])?;
        let right = Hand::try_from(value[2])?;

        Ok(Self { left, right })
    }
}

impl Round {
    fn score(&self) -> u32 {
        self.right.fight(&self.left) + self.right.score()
    }
}

fn solve_part_1(input: &str) -> anyhow::Result<()> {
    let rounds: u32 = input
        .split('\n')
        .map(|c| Round::try_from(c.as_bytes()).unwrap().score())
        .sum();

    println!("Day 2-1: {:?}", rounds);
    Ok(())
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl TryFrom<u8> for RoundResult {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Lose),
            b'Y' => Ok(Draw),
            b'Z' => Ok(Win),
            _ => Err(()),
        }
    }
}

impl RoundResult {
    fn rig(&self, hand: &Hand) -> Hand {
        match (self, hand) {
            (Win, Scissors) => Rock,
            (Win, Paper) => Scissors,
            (Win, Rock) => Paper,
            (Lose, Scissors) => Paper,
            (Lose, Paper) => Rock,
            (Lose, Rock) => Scissors,
            _ => hand.clone(),
        }
    }
}

struct RiggedRound {
    left: Hand,
    right: RoundResult,
}

impl TryFrom<&[u8]> for RiggedRound {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            return Err(());
        }

        let left = Hand::try_from(value[0])?;
        let right = RoundResult::try_from(value[2])?;

        Ok(Self { left, right })
    }
}

impl RiggedRound {
    fn play(&self) -> u32 {
        let right = self.right.rig(&self.left);

        right.fight(&self.left) + right.score()
    }
}

fn solve_part_2(input: &str) -> anyhow::Result<()> {
    let rounds: u32 = input
        .split('\n')
        .map(|c| RiggedRound::try_from(c.as_bytes()).unwrap().play())
        .sum();

    println!("Day 2-2: {:?}", rounds);
    Ok(())
}
