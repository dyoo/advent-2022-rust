use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct HeightMap(Vec<Vec<u8>>);

impl FromStr for HeightMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HeightMap(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| {
                            ch.to_digit(10)
                                .map(|x| x as u8)
                                .ok_or(format!("not a digit: {}", ch))
                        })
                        .collect::<Result<Vec<u8>, _>>()
                })
                .collect::<Result<Vec<Vec<u8>>, _>>()?,
        ))
    }
}

impl HeightMap {
    fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width()).flat_map(|x| (0..self.height()).map(move |y| (x, y)))
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.0.get(y)?.get(x).copied()
    }

    fn right(&self, x: usize, y: usize) -> WalkToEdge {
        WalkToEdge::new(self, x, y, 1, 0)
    }

    fn left(&self, x: usize, y: usize) -> WalkToEdge {
        WalkToEdge::new(self, x, y, -1, 0)
    }

    fn up(&self, x: usize, y: usize) -> WalkToEdge {
        WalkToEdge::new(self, x, y, 0, -1)
    }

    fn down(&self, x: usize, y: usize) -> WalkToEdge {
        WalkToEdge::new(self, x, y, 0, 1)
    }
}

#[derive(Debug)]
struct WalkToEdge<'a> {
    height_map: &'a HeightMap,
    current_x: usize,
    current_y: usize,
    delta_x: isize,
    delta_y: isize,
}

impl<'a> WalkToEdge<'a> {
    fn new(
        height_map: &'a HeightMap,
        current_x: usize,
        current_y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> Self {
        WalkToEdge {
            height_map,
            current_x,
            current_y,
            delta_x,
            delta_y,
        }
    }
}

impl<'a> Iterator for WalkToEdge<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(new_x) = self.current_x.checked_add_signed(self.delta_x) {
            self.current_x = new_x;
        } else {
            return None;
        }

        if let Some(new_y) = self.current_y.checked_add_signed(self.delta_y) {
            self.current_y = new_y;
        } else {
            return None;
        }

        let row = self.height_map.0.get(self.current_y)?;
        let cell = row.get(self.current_x)?;

        Some(*cell)
    }
}

fn is_visible(hmap: &HeightMap, x: usize, y: usize) -> bool {
    if let Some(h) = hmap.get(x, y) {
        for mut slice in [
            hmap.left(x, y),
            hmap.right(x, y),
            hmap.up(x, y),
            hmap.down(x, y),
        ] {
            if !slice.any(|other| other >= h) {
                return true;
            }
        }
    }
    false
}

fn part_1(hmap: &HeightMap) -> usize {
    hmap.coords()
        .filter(|&(x, y)| is_visible(hmap, x, y))
        .count()
}

#[test]
fn test_part_1() {
    let example_map = HeightMap(vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ]);

    assert_eq!(part_1(&example_map), 21);
}

fn scenic_score(hmap: &HeightMap, x: usize, y: usize) -> usize {
    if let Some(h) = hmap.get(x, y) {
        [
            hmap.left(x, y),
            hmap.right(x, y),
            hmap.up(x, y),
            hmap.down(x, y),
        ]
        .map(|slice| {
            let mut count = 0;
            for other in slice {
                count += 1;
                if other >= h {
                    break;
                }
            }
            count
        })
        .into_iter()
        .product()
    } else {
        1
    }
}

#[test]
fn test_scenic_score() -> Result<(), Box<dyn Error>> {
    let hmap: HeightMap = "30373
25512
65332
33549
35390"
        .parse()?;
    assert_eq!(scenic_score(&hmap, 2, 3), 8);
    Ok(())
}

fn part_2(hmap: &HeightMap) -> Option<usize> {
    hmap.coords().map(|(x, y)| scenic_score(hmap, x, y)).max()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("adventofcode.com_2022_day_8_input.txt").expect("input");
    let hmap: HeightMap = input.parse()?;
    println!("Part 1: {}", part_1(&hmap));

    println!("Part 2: {}", part_2(&hmap).ok_or("empty")?);
    Ok(())
}
