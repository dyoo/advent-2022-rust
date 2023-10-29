use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Pos { x, y, z }
    }

    fn faces(&self) -> Vec<Pos> {
        let mut result = Vec::new();
        for i in [-1, 1] {
            result.push(Pos::new(self.x + i, self.y, self.z));
        }
        for j in [-1, 1] {
            result.push(Pos::new(self.x, self.y + j, self.z));
        }
        for k in [-1, 1] {
            result.push(Pos::new(self.x, self.y, self.z + k));
        }
        result
    }
}

fn surface_area_1(cubes: &[Pos]) -> usize {
    let cubes_set = cubes.iter().copied().collect::<HashSet<Pos>>();

    // The number of exposed faces are those that are facing empty space
    // (not occupied by an existing cube).
    cubes
        .iter()
        .flat_map(Pos::faces)
        .filter(|c| !cubes_set.contains(c))
        .count()
}

fn surface_area_2(cubes: &[Pos]) -> usize {
    let cubes_set = cubes.iter().copied().collect::<HashSet<Pos>>();

    // The number of exposed faces are those that are facing empty space
    // * not occupied by an existing cube
    // * can reach the outside.
    let candidate_faces = cubes
        .iter()
        .flat_map(Pos::faces)
        .filter(|c| !cubes_set.contains(c));

    // We need to check each face whether it can reach the outside.
    // We can use DFS, using faces as our neighbor function, till we either
    // reach a boundary (min-1, max+1 for any coordinate) or we exhaust.

    0
}

struct FloodingBoundarySearch<'a> {
    cubes: &'a HashSet<Pos>,
    cache: HashMap<Pos, bool>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    z_bounds: (i32, i32),
}

impl<'a> FloodingBoundarySearch<'a> {
    fn new(cubes: &'a HashSet<Pos>) -> Self {
        let x_bounds = (
            cubes.iter().map(|c| c.x).min().unwrap() - 1,
            cubes.iter().map(|c| c.x).max().unwrap() - 1,
        );
        let y_bounds = (
            cubes.iter().map(|c| c.y).min().unwrap() - 1,
            cubes.iter().map(|c| c.y).max().unwrap() - 1,
        );
        let z_bounds = (
            cubes.iter().map(|c| c.z).min().unwrap() - 1,
            cubes.iter().map(|c| c.z).max().unwrap() - 1,
        );
        Self {
            cubes,
            cache: HashMap::new(),
            x_bounds,
            y_bounds,
            z_bounds,
        }
    }
}

fn parse(s: &str) -> Vec<Pos> {
    s.trim()
        .lines()
        .map(|l| {
            let vals: Vec<i32> = l.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            Pos::new(vals[0], vals[1], vals[2])
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("part 1: {}", surface_area_1(&parse(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_area_1_small_example() {
        assert_eq!(
            surface_area_1(&vec![Pos::new(1, 1, 1), Pos::new(2, 1, 1)]),
            10
        );
    }

    const SMALL_INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_parse() {
        let positions = parse(SMALL_INPUT);
        assert_eq!(
            positions,
            vec![
                Pos::new(2, 2, 2),
                Pos::new(1, 2, 2),
                Pos::new(3, 2, 2),
                Pos::new(2, 1, 2),
                Pos::new(2, 3, 2),
                Pos::new(2, 2, 1),
                Pos::new(2, 2, 3),
                Pos::new(2, 2, 4),
                Pos::new(2, 2, 6),
                Pos::new(1, 2, 5),
                Pos::new(3, 2, 5),
                Pos::new(2, 1, 5),
                Pos::new(2, 3, 5),
            ]
        );
    }

    #[test]
    fn surface_area_small_input() {
        let positions = parse(SMALL_INPUT);
        assert_eq!(surface_area_1(&positions), 64);
    }
}
