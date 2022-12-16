use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
    ops::RangeInclusive,
};

use anyhow::Result;

use crate::{Coord, Errors};

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("/Users/son/workspace/rust-aoc/y2022/inputs/day15.txt")?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let deployments: Vec<SensorDeployment> = input
        .split('\n')
        .filter_map(|s| SensorDeployment::try_from(s).ok())
        .collect();

    let ranges: Vec<Option<RangeInclusive<isize>>> = deployments
        .iter()
        .map(|d| d.get_range_for_row(2000000))
        .collect();

    let ranges: Vec<RangeInclusive<isize>> = ranges.iter().filter_map(|f| f.clone()).collect();
    let merged_ranges = merge_ranges(ranges);

    let number_of_spots: isize = merged_ranges
        .iter()
        .map(|r| (*r.start() - *r.end()).abs())
        .sum();

    println!("Day 15-1: {:?}", number_of_spots);
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let deployments: Vec<SensorDeployment> = input
        .split('\n')
        .filter_map(|s| SensorDeployment::try_from(s).ok())
        .collect();

    let min_x = 0;
    let min_y = deployments.iter().map(|d| d.sensor.y).min().unwrap();
    let max_x = 4000000;
    let max_y = deployments.iter().map(|d| d.sensor.y).max().unwrap();

    let mut row_ranges: HashMap<isize, Vec<RangeInclusive<isize>>> = HashMap::new();
    for i in min_y..=max_y {
        let ranges: Vec<RangeInclusive<isize>> = deployments
            .iter()
            .filter_map(|d| d.get_range_for_row(i))
            .collect();
        let ranges = merge_ranges(ranges);
        let ranges = normalize_ranges(ranges, min_x, max_x);
        if ranges.len() == 2 {
            row_ranges.insert(i, ranges);
        }
    }

    let key = row_ranges.keys().last().unwrap();
    let value = row_ranges.get(key).unwrap();
    let value = value[0].end() + 1;

    println!("Day 15-2: {:?}", value * 4000000 + key);
    Ok(())
}

fn normalize_ranges(
    ranges: Vec<RangeInclusive<isize>>,
    lower_bound: isize,
    higher_bound: isize,
) -> Vec<RangeInclusive<isize>> {
    ranges
        .iter()
        .filter(|r| !(*r.end() < lower_bound || *r.start() > higher_bound))
        .map(|r| {
            let start = max(*r.start(), lower_bound);
            let end = min(*r.end(), higher_bound);
            start..=end
        })
        .collect()
}

fn merge_ranges(ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    let mut ranges = ranges;
    ranges.sort_by(|a, b| {
        let start_cmp = a.start().cmp(b.start());
        let end_cmp = a.end().cmp(b.end());

        if !start_cmp.is_eq() {
            return start_cmp;
        }

        return end_cmp;
    });

    let mut merged_ranges: Vec<RangeInclusive<isize>> = vec![];
    let mut idx = 0;
    let mut cur = 1;

    merged_ranges.push(ranges[idx].clone());
    while cur < ranges.len() {
        if ranges[cur].start() > merged_ranges[idx].end() {
            merged_ranges.push(ranges[cur].clone());
            idx += 1;
            cur += 1;
            continue;
        }

        let end = max(*merged_ranges[idx].end(), *ranges[cur].end());

        merged_ranges[idx] = *merged_ranges[idx].start()..=end;

        cur += 1;
    }

    merged_ranges
}

#[derive(Debug)]
struct SensorDeployment {
    sensor: Coord,
    beacon: Coord,
}

impl TryFrom<&str> for SensorDeployment {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let input: Vec<Coord> = value
            .split(':')
            .map(|s| {
                s.replace("Sensor at ", "")
                    .replace("closest beacon is at", "")
            })
            .filter_map(|s| parse_coord(&s).ok())
            .collect();

        Ok(SensorDeployment {
            sensor: input[0],
            beacon: input[1],
        })
    }
}

fn parse_coord(input: &str) -> Result<Coord, Errors> {
    let input: Vec<isize> = input
        .trim()
        .split(", ")
        .flat_map(|s| s.split('=').filter_map(|n| n.parse::<isize>().ok()))
        .collect();

    if input.len() != 2 {
        return Err(Errors::ParseError("Invalid Coord, can't parse".into()));
    }

    Ok(Coord {
        x: input[0],
        y: input[1],
    })
}

impl SensorDeployment {
    fn get_range_for_row(&self, row: isize) -> Option<RangeInclusive<isize>> {
        let size = self.find_size_of_diamond();

        if self.sensor.y + size < row || self.sensor.y - size > row {
            return None;
        }

        let diff = (self.sensor.y - row).abs();
        let units = (size - diff).abs();

        Some(RangeInclusive::new(
            self.sensor.x - units,
            self.sensor.x + units,
        ))
    }

    fn find_size_of_diamond(&self) -> isize {
        if self.sensor.x == self.beacon.x {
            (self.sensor.y - self.beacon.y).abs()
        } else if self.sensor.y == self.beacon.y {
            (self.sensor.x - self.beacon.x).abs()
        } else {
            let x_axis = (self.sensor.x - self.beacon.x).abs();
            let y_axis = (self.sensor.y - self.beacon.y).abs();

            y_axis + x_axis
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Coord;

    use super::SensorDeployment;

    #[test]
    fn test_find_diamond_size() {
        let deloyment = SensorDeployment {
            sensor: Coord::new(8, 7),
            beacon: Coord::new(2, 10),
        };

        assert_eq!(deloyment.find_size_of_diamond(), 9);
    }

    #[test]
    fn test_get_range_for_row() {
        let deployment = SensorDeployment {
            sensor: Coord::new(8, 7),
            beacon: Coord::new(2, 10),
        };

        assert_eq!(deployment.get_range_for_row(10), Some(2..=14));
        assert_eq!(deployment.get_range_for_row(11), Some(3..=13));
        assert_eq!(deployment.get_range_for_row(16), Some(8..=8));
        assert_eq!(deployment.get_range_for_row(7), Some(-1..=17));
        assert_eq!(deployment.get_range_for_row(-2), Some(8..=8));
        assert_eq!(deployment.get_range_for_row(17), None);
        assert_eq!(deployment.get_range_for_row(-3), None);

        let deployment = SensorDeployment {
            sensor: Coord::new(16, 7),
            beacon: Coord::new(15, 3),
        };

        assert_eq!(deployment.get_range_for_row(3), Some(15..=17));
    }

    #[test]
    fn test_weird_bug_for_range() {
        let deployment = SensorDeployment {
            sensor: Coord::new(12, 14),
            beacon: Coord::new(10, 16),
        };

        assert_eq!(deployment.get_range_for_row(13), Some(9..=15));
    }
}
