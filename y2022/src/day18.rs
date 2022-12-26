use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::Result;

use crate::Errors;

pub fn exec() -> Result<()> {
    let input_example = fs::read_to_string("./inputs/day18_example.txt")?;
    let input = fs::read_to_string("./inputs/day18.txt")?;

    solve_part_1(&input_example)?;
    solve_part_1(&input)?;
    solve_part_2(&input)?;

    Ok(())
}

fn solve_part_1(input: &str) -> Result<()> {
    let cubes: Vec<Cube> = input.trim().split('\n').map(|s| Cube::from(s)).collect();
    let mut plane = Plane::new(cubes);

    plane.link_connected_cubes()?;

    println!("Day 18-1: {}", plane.calculate_surface_area());
    Ok(())
}

fn solve_part_2(input: &str) -> Result<()> {
    println!("Day 18-2: {}", "");
    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Cube(isize, isize, isize);

struct Plane {
    shapes: HashMap<Cube, Cube>,
    rank: HashMap<Cube, usize>,
}

impl Plane {
    fn new(cubes: impl IntoIterator<Item = Cube>) -> Self {
        let mut shapes = HashMap::new();
        let mut rank = HashMap::new();
        cubes.into_iter().for_each(|cube| {
            shapes.insert(cube, cube);
            rank.insert(cube, 1);
        });

        Plane { shapes, rank }
    }

    fn find(&mut self, cube: Cube) -> Result<Cube> {
        let p = self
            .shapes
            .get(&cube)
            .ok_or(Errors::InvalidState("Root doesn't exist".into()))?
            .clone();

        if cube == p {
            return Ok(cube);
        }

        let p_root = self.find(p)?;
        self.shapes.insert(cube, p_root);

        Ok(p_root)
    }

    fn get_rank(&self, cube: Cube) -> Result<usize, Errors> {
        self.rank
            .get(&cube)
            .ok_or(Errors::InvalidState("Rank doesn't exist".into()))
            .map(|r| *r)
    }

    fn union(&mut self, left: Cube, right: Cube) -> Result<()> {
        let mut left_root = self.find(left)?;
        let mut right_root = self.find(right)?;

        if left_root != right_root {
            let left_rank = self.get_rank(left_root)?;
            let right_rank = self.get_rank(right_root)?;

            if left_rank < right_rank {
                let tmp = right_root;
                right_root = left_root;
                left_root = tmp;
            }

            self.shapes.insert(right_root, left_root);
            if left_rank == right_rank {
                self.rank.entry(left_root).and_modify(|rank| *rank += 1);
            }
        }

        Ok(())
    }

    fn get_shapes(&mut self) -> Result<HashMap<Cube, Vec<Cube>>> {
        let mut results: HashMap<Cube, Vec<Cube>> = HashMap::new();
        let cubes: Vec<Cube> = self.shapes.keys().copied().collect();
        for cube in cubes {
            let root = self.find(cube)?;
            results
                .entry(root)
                .and_modify(|v| v.push(cube))
                .or_insert_with(|| vec![cube]);
        }

        Ok(results)
    }

    fn link_connected_cubes(&mut self) -> Result<()> {
        let cubes: HashSet<Cube> = self.shapes.keys().copied().collect();
        let mut sorted_cubes: Vec<Cube> = self.shapes.keys().copied().collect::<Vec<Cube>>();
        sorted_cubes.sort();
        for cube in &sorted_cubes {
            for neighbor in cube.neighbors().iter().filter(|n| cubes.contains(n)) {
                self.union(*cube, *neighbor)?;
            }
        }

        Ok(())
    }
}

impl Cube {
    fn neighbors(&self) -> Vec<Cube> {
        vec![
            Cube(self.0 - 1, self.1, self.2),
            Cube(self.0 + 1, self.1, self.2),
            Cube(self.0, self.1 - 1, self.2),
            Cube(self.0, self.1 + 1, self.2),
            Cube(self.0, self.1, self.2 - 1),
            Cube(self.0, self.1, self.2 + 1),
        ]
    }
}

trait SurfaceArea {
    fn calculate_surface_area(&self) -> usize;
}

trait SurfaceAreaWithMutation {
    fn calculate_surface_area(&mut self) -> usize;
}

impl SurfaceArea for Vec<Cube> {
    fn calculate_surface_area(&self) -> usize {
        let set: HashSet<Cube> = HashSet::from_iter(self.iter().copied());
        let mut results = 0;

        for cube in self {
            let neighbors = HashSet::from_iter(cube.neighbors());
            results += neighbors.len() - neighbors.intersection(&set).count();
        }

        results
    }
}

impl SurfaceAreaWithMutation for Plane {
    fn calculate_surface_area(&mut self) -> usize {
        if self.link_connected_cubes().is_err() {
            return 0;
        }

        let shapes = self.get_shapes().unwrap_or_default();

        shapes
            .iter()
            .map(|(_k, v)| v.calculate_surface_area())
            .sum()
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let split: Vec<isize> = value
            .split(',')
            .map(|c| c.parse::<isize>().unwrap_or_default())
            .collect();

        Cube(split[0], split[1], split[2])
    }
}

// ########### Tests
#[cfg(test)]
mod tests {
    use crate::day18::{Cube, SurfaceArea};
    use anyhow::Result;

    use super::Plane;

    #[test]
    fn test_union_find_shapes() -> Result<()> {
        let mut plane = Plane::new(vec![Cube(1, 1, 1), Cube(2, 1, 1), Cube(3, 2, 1)]);

        plane.union(Cube(1, 1, 1), Cube(2, 1, 1))?;

        let shapes = plane.get_shapes()?;

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes.get(&Cube(1, 1, 1)).unwrap().len(), 2);
        assert_eq!(shapes.get(&Cube(3, 2, 1)).unwrap().len(), 1);

        plane.union(Cube(2, 1, 1), Cube(3, 2, 1))?;

        let shapes = plane.get_shapes()?;
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes.get(&Cube(1, 1, 1)).unwrap().len(), 3);

        Ok(())
    }

    #[test]
    fn test_link_cubes() -> Result<()> {
        let mut plane = Plane::new(vec![Cube(1, 1, 1), Cube(2, 1, 1), Cube(3, 2, 1)]);

        plane.link_connected_cubes()?;

        let shapes = plane.get_shapes()?;

        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes.get(&Cube(1, 1, 1)).unwrap().len(), 2);
        assert_eq!(shapes.get(&Cube(3, 2, 1)).unwrap().len(), 1);

        Ok(())
    }

    #[test]
    fn test_surface_area() -> Result<()> {
        let mut plane = Plane::new(vec![Cube(1, 1, 1), Cube(2, 1, 1), Cube(3, 2, 1)]);

        plane.link_connected_cubes()?;

        let shapes = plane.get_shapes()?;

        assert_eq!(
            shapes.get(&Cube(1, 1, 1)).unwrap().calculate_surface_area(),
            10
        );

        Ok(())
    }
}
