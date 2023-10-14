use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn row(&self) -> usize {
        self.0
    }
    fn col(&self) -> usize {
        self.1
    }
}

#[derive(Debug)]
struct HeightMap {
    data: Vec<u8>,
    cols: usize,
    rows: usize,
}

impl HeightMap {
    fn find(&self, pred: fn(u8) -> bool) -> Option<Pos> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.at(Pos(row, col))?;
                if pred(ch) {
                    return Some(Pos(row, col));
                }
            }
        }
        None
    }

    fn find_all(&self, pred: &dyn Fn(u8) -> bool) -> Vec<Pos> {
        let mut results = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(ch) = self.at(Pos(row, col)) {
                    if pred(ch) {
                        results.push(Pos(row, col));
                    }
                }
            }
        }
        results
    }

    fn at(&self, p: Pos) -> Option<u8> {
        self.data.get(self.cols * p.row() + p.col()).copied()
    }

    fn height(&self, p: Pos) -> Option<u8> {
        let ch = self.at(p)?;
        match ch {
            b'S' => Some(0),
            b'E' => Some(25),
            b'a'..=b'z' => Some(ch - b'a'),
            _ => None,
        }
    }

    // Returns list of neighbor positions in-bounds of the heightmap
    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        // left
        if p.col().checked_sub(1).is_some() {
            result.push(Pos(p.row(), p.col() - 1));
        }
        // right
        if p.col() + 1 < self.cols {
            result.push(Pos(p.row(), p.col() + 1));
        }
        // up
        if p.row().checked_sub(1).is_some() {
            result.push(Pos(p.row() - 1, p.col()));
        }
        // down
        if p.row() + 1 < self.rows {
            result.push(Pos(p.row() + 1, p.col()));
        }

        result
    }
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::<u8>::new();
        let mut cols = 0;
        let mut rows = 0;
        for line in s.split_whitespace().map(|s| s.as_bytes()) {
            // Check character validity:
            for ch in line {
                match ch {
                    b'S' | b'E' | b'a'..=b'z' => {}
                    _ => {
                        return Err(format!("Invalid height {}", ch));
                    }
                }
            }
            data.extend(line);
            cols = line.len();
            rows += 1;
        }
        Ok(Self { data, cols, rows })
    }
}

fn part_1(h: &HeightMap) -> Option<u32> {
    search(h, h.find(|ch| ch == b'S'))
}

fn part_2(h: &HeightMap) -> Option<u32> {
    search(h, h.find_all(&|p| p == b'a' || p == b'S'))
}

fn search(h: &HeightMap, starting_positions: impl IntoIterator<Item = Pos>) -> Option<u32> {
    // Keep a queue of (position, distance) pairs.
    let mut queue = VecDeque::<(Pos, u32)>::new();
    for starting in starting_positions {
        queue.push_back((starting, 0));
    }

    let mut visited = HashSet::<Pos>::new();

    while let Some((p, dist)) = queue.pop_front() {
        // Skip if we've been here before.
        if visited.contains(&p) {
            continue;
        }

        // Terminate search early if we hit the end.
        if h.at(p) == Some(b'E') {
            return Some(dist);
        }

        // Mark the visit and queue up the neighbors that we can visit.
        visited.insert(p);

        let p_height = h.height(p)?;
        let candidates = h
            .neighbors(p)
            .into_iter()
            .filter(|&candidate| {
                if let Some(candidate_height) = h.height(candidate) {
                    // We can either descend, stay at the same height, or
                    // climb up by one.
                    candidate_height <= (p_height + 1)
                } else {
                    false
                }
            })
            .filter(|candidate| !visited.contains(candidate));

        queue.extend(candidates.map(|candidate| (candidate, dist + 1)));
    }

    None
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let h: HeightMap = input.parse().unwrap();
    println!("part 1: {:?}", part_1(&h));
    println!("part 2: {:?}", part_2(&h));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_MAP: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_from_str() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        assert_eq!(h.rows, 5);
        assert_eq!(h.cols, 8);
    }

    #[test]
    fn test_at() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        assert_eq!(h.at(Pos(0, 0)), Some(b'S'));
        assert_eq!(h.at(Pos(1, 1)), Some(b'b'));
        assert_eq!(h.at(Pos(2, 2)), Some(b'c'));
        assert_eq!(h.at(Pos(3, 3)), Some(b't'));
        assert_eq!(h.at(Pos(4, 4)), Some(b'f'));
        assert_eq!(h.at(Pos(5, 5)), None);
    }

    #[test]
    fn test_height() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        assert_eq!(h.height(Pos(0, 0)), Some(0));
        assert_eq!(h.height(Pos(0, 1)), Some(0));
        assert_eq!(h.height(Pos(0, 2)), Some(1));
        assert_eq!(h.height(Pos(0, 3)), Some(16));
    }

    #[test]
    fn test_find() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        assert_eq!(h.find(|ch| ch == b'S'), Some(Pos(0, 0)));
    }

    #[test]
    fn test_neighbors() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        // Upper left corner
        assert_eq!(h.neighbors(Pos(0, 0)), vec![Pos(0, 1), Pos(1, 0)]);

        assert_eq!(
            h.neighbors(Pos(1, 1)),
            vec![Pos(1, 0), Pos(1, 2), Pos(0, 1), Pos(2, 1)]
        );

        // Bottom right corner
        assert_eq!(h.neighbors(Pos(4, 7)), vec![Pos(4, 6), Pos(3, 7)]);
    }

    #[test]
    fn test_part_1() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        assert_eq!(part_1(&h), Some(31));
    }

    #[test]
    fn test_find_all() {
        let h: HeightMap = SMALL_MAP.parse().expect("Oops");
        assert_eq!(
            h.find_all(&|p| p == b'a' || p == b'S'),
            vec![
                Pos(0, 0),
                Pos(0, 1),
                Pos(1, 0),
                Pos(2, 0),
                Pos(3, 0),
                Pos(4, 0)
            ]
        );
    }
}
