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
    for i in 0..=4000000 {
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

    // let mut missed_spots = HashSet::new();
    // for x in min_x..max_x {
    //     for y in min_y..max_y {
    //         if let Some(ranges) = row_ranges.get(&y) {
    //             if !ranges.iter().any(|r| r.contains(&x)) {
    //                 missed_spots.insert(Coord::new(x, y));
    //             }
    //         }
    //     }
    // }

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
    fn minimum_coord(&self) -> Coord {
        Coord::new(
            min(self.sensor.x, self.beacon.x),
            min(self.sensor.y, self.beacon.y),
        )
    }

    fn maximum_coord(&self) -> Coord {
        Coord::new(
            max(self.sensor.x, self.beacon.x),
            max(self.sensor.y, self.beacon.y),
        )
    }

    fn get_row(&self, row: isize) -> HashSet<Coord> {
        self.sensor
            .coords_for_diamond_row(self.find_size_of_diamond(), row)
    }

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

    fn get_diamond(&self) -> HashSet<Coord> {
        self.sensor.coords_for_diamond(self.find_size_of_diamond())
    }

    fn within_range(&self, coord: Coord) -> bool {
        let diamond_size = self.find_size_of_diamond();
        let diamond_coords = self.sensor.coords_for_diamond(diamond_size);

        diamond_coords.contains(&coord)
    }

    fn find_size_of_diamond(&self) -> isize {
        if self.sensor.x == self.beacon.x {
            (self.sensor.y - self.beacon.y).abs()
        } else if self.sensor.y == self.beacon.y {
            (self.sensor.x - self.beacon.x).abs()
        } else {
            let mut size = 0;

            let direction = self.sensor.direction_towards(&self.beacon);

            let x_axis = (self.sensor.x - self.beacon.x).abs();
            let y_axis = (self.sensor.y - self.beacon.y).abs();

            y_axis + x_axis
            // let mut coord = self.sensor;
            // coord.x += direction.x;

            // while !coord.is_on_the_diag(&self.beacon) {
            //     coord.x += direction.x;
            // }
            // size = max(size, (self.sensor.x - coord.x).abs());

            // let mut coord = self.sensor;
            // coord.y += direction.y;
            // while !coord.is_on_the_diag(&self.beacon) {
            //     coord.y += direction.y;
            // }
            // size = max(size, (self.sensor.y - coord.y).abs());

            // size
        }
    }
}

impl Coord {
    fn is_on_the_diag(&self, other: &Coord) -> bool {
        (self.x - other.x).abs() == (self.y - other.y).abs()
    }

    fn coords_for_diamond_row(&self, size: isize, row: isize) -> HashSet<Coord> {
        if (self.y < row && self.y + size < row) || (self.y > row && self.y - size > row) {
            return HashSet::new();
        }

        let mut coords = HashSet::new();
        for i in (0..=(size - (self.y - row).abs())).rev() {
            coords.insert(Coord {
                x: self.x - i,
                y: row,
            });
            coords.insert(Coord {
                x: self.x + i,
                y: row,
            });
        }

        coords
    }

    fn coords_for_diamond(&self, size_on_axis: isize) -> HashSet<Coord> {
        let mut coords = HashSet::new();

        // coords for the top
        for j in 0..=size_on_axis {
            for i in (0..=(size_on_axis - j)).rev() {
                coords.insert(Coord {
                    x: self.x - i,
                    y: self.y + j,
                });
                coords.insert(Coord {
                    x: self.x + i,
                    y: self.y + j,
                });
            }
        }

        // coords for the bottom
        for j in 0..=size_on_axis {
            for i in (0..=(size_on_axis - j)).rev() {
                coords.insert(Coord {
                    x: self.x - i,
                    y: self.y - j,
                });
                coords.insert(Coord {
                    x: self.x + i,
                    y: self.y - j,
                });
            }
        }

        coords
    }
}

#[cfg(test)]
mod test {
    use crate::Coord;

    use super::SensorDeployment;

    #[test]
    fn test_diamond_shape() {
        let coord = Coord::new(5, 5);

        let diamond = coord.coords_for_diamond(3);
        assert_eq!(diamond.len(), 25);

        let diamond = coord.coords_for_diamond(2);
        assert_eq!(diamond.len(), 13);
    }

    #[test]
    fn test_find_diamond_size() {
        let deloyment = SensorDeployment {
            sensor: Coord::new(8, 7),
            beacon: Coord::new(2, 10),
        };

        assert_eq!(deloyment.find_size_of_diamond(), 9);
    }

    #[test]
    fn test_within_range() {
        let deloyment = SensorDeployment {
            sensor: Coord::new(8, 7),
            beacon: Coord::new(2, 10),
        };

        assert!(deloyment.within_range(Coord::new(6, 14)));
        assert!(!deloyment.within_range(Coord::new(5, 14)));
    }

    #[test]
    fn test_get_diamond() {
        let deployment = SensorDeployment {
            sensor: Coord::new(10, 20),
            beacon: Coord::new(10, 16),
        };

        assert_eq!(deployment.get_diamond().len(), 41);

        let deployment = SensorDeployment {
            sensor: Coord::new(14, 3),
            beacon: Coord::new(15, 3),
        };

        assert_eq!(deployment.get_diamond().len(), 5);
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

    #[test]
    fn test_get_coords_for_row() {
        let coord = Coord::new(8, 7);

        let coords = coord.coords_for_diamond_row(9, 16);
        assert_eq!(coords.len(), 1);

        let coords = coord.coords_for_diamond_row(9, 15);
        assert_eq!(coords.len(), 3);

        let coords = coord.coords_for_diamond_row(9, 5);
        assert_eq!(coords.len(), 15);

        let deloyment = SensorDeployment {
            sensor: Coord::new(8, 7),
            beacon: Coord::new(2, 10),
        };

        assert_eq!(deloyment.get_row(16).len(), 1);
        assert!(deloyment.get_row(16).contains(&Coord::new(8, 16)));
        assert!(deloyment.get_row(15).contains(&Coord::new(8, 15)));
        assert!(deloyment.get_row(15).contains(&Coord::new(7, 15)));
        assert!(deloyment.get_row(15).contains(&Coord::new(9, 15)));
    }
}
