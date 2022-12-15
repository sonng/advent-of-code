use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

use anyhow::Result;

use crate::{Coord, Errors};

pub fn exec() -> Result<()> {
    let input = fs::read_to_string("./inputs/day15.txt")?;
    // solve_part_1(&input)?;
    solve_part_2(&input)?;
    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let deployments: Vec<SensorDeployment> = input
        .split('\n')
        .filter_map(|s| SensorDeployment::try_from(s).ok())
        .collect();

    let mut other_points = HashSet::new();
    deployments.iter().for_each(|d| {
        other_points.insert(d.sensor);
        other_points.insert(d.beacon);
    });

    let spots = deployments.iter().flat_map(|d| d.get_row(2000000));

    let impossible: HashSet<Coord> = spots.filter(|s| !other_points.contains(s)).collect();

    println!("Day 15-1: {:?}", impossible.len());
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    let deployments: Vec<SensorDeployment> = input
        .split('\n')
        .filter_map(|s| SensorDeployment::try_from(s).ok())
        .collect();

    let all_spots: HashSet<Coord> = deployments.iter().flat_map(|d| d.get_diamond()).collect();

    let mut missed_spots = HashSet::new();
    for x in 0..20 {
        for y in 0..20 {
            let point = Coord::new(x as isize, y as isize);
            if !all_spots.contains(&point) {
                missed_spots.insert(point);
            }
        }
    }

    println!("Day 15-2: {:?}", missed_spots);
    Ok(())
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
            (self.sensor.x - self.beacon.x).abs()
        } else if self.sensor.y == self.beacon.y {
            (self.sensor.y - self.beacon.y).abs()
        } else {
            let mut size = 0;

            let direction = self.sensor.direction_towards(&self.beacon);

            let mut coord = self.sensor;
            while !coord.is_on_the_diag(&self.beacon) {
                coord.x += direction.x;
            }
            size = max(size, (self.sensor.x - coord.x).abs());

            let mut coord = self.sensor;
            while !coord.is_on_the_diag(&self.beacon) {
                coord.y += direction.y;
            }
            size = max(size, (self.sensor.y - coord.y).abs());

            size
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
