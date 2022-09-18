use std::cmp::min;
use std::fs;
use std::str::FromStr;
use anyhow::{ Result, anyhow };

pub fn execute() -> Result<()> {
    let file = fs::read_to_string("./inputs/2.txt")?;
    let boxes: Vec<Rect> = file.split("\n")
        .map(|i| Rect::from_str(i).unwrap())
        .collect();

    println!("# Day 2");
    println!("Part 1: {:?}", solve_part_1(&boxes));
    println!("Part 2: {:?}", solve_part_2(&boxes));

    Ok(())
}

#[derive(Debug)]
struct Rect {
    l: usize,
    w: usize,
    h: usize
}

impl Rect {
    fn surface_area(&self) -> usize {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn min_face_area(&self) -> usize {
        let side1 = self.l * self.w;
        let side2 = self.w * self.h;
        let side3 = self.h * self.l;

        min(side1, min(side2, side3))
    }

    fn min_perimeter(&self) -> usize {
        min(2 * self.l + 2 * self.w, min(2 * self.l + 2 * self.h, 2 * self.w + 2 * self.h))
    }

    fn cubic(&self) -> usize {
        self.l * self.w * self.h
    }
}

impl FromStr for Rect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lengths: Vec<usize> = s.split('x')
            .map(|l| l.parse::<usize>().expect("invalid number"))
            .collect();

        if lengths.len() != 3 {
            return Err(anyhow!("Expected {l}x{w}x{h}".to_string()));
        }

        Ok(Rect { l: lengths[0], w: lengths[1], h: lengths[2] })
    }
}

fn solve_part_1(boxes: &Vec<Rect>) -> usize {
    boxes.iter().fold(0, |acc, rect| {
        acc + (rect.surface_area() + rect.min_face_area())
    })
}

fn solve_part_2(boxes: &Vec<Rect>) -> usize {
    boxes.iter().fold(0, |acc, rect| {
        acc + (rect.min_perimeter() + rect.cubic())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Rect { l: 2, w: 3, h: 4}.surface_area(), 52);
        assert_eq!(Rect { l: 2, w: 3, h: 4}.min_face_area(), 6);
        assert_eq!(Rect { l: 1, w: 1, h: 10}.surface_area(), 42);
        assert_eq!(Rect { l: 1, w: 1, h: 10}.min_face_area(), 1);
        assert_eq!(solve_part_1(&vec![Rect { l: 2, w: 3, h: 4}]), 58);
        assert_eq!(solve_part_1(&vec![Rect { l: 1, w: 1, h: 10}]), 43);


        assert_eq!(solve_part_1(&vec![Rect { l: 2, w: 3, h: 4}, Rect { l: 1, w: 1, h: 10}]), 101);


        assert_eq!(Rect { l: 2, w: 3, h: 4}.min_perimeter(), 10);
        assert_eq!(Rect { l: 2, w: 3, h: 4}.cubic(), 24);
        assert_eq!(Rect { l: 1, w: 1, h: 10}.min_perimeter(), 4);
        assert_eq!(Rect { l: 1, w: 1, h: 10}.cubic(), 10);
        assert_eq!(solve_part_2(&vec![Rect { l: 2, w: 3, h: 4}]), 34);
        assert_eq!(solve_part_2(&vec![Rect { l: 1, w: 1, h: 10}]), 14);


        assert_eq!(solve_part_2(&vec![Rect { l: 2, w: 3, h: 4}, Rect { l: 1, w: 1, h: 10}]), 48);
    }
}
