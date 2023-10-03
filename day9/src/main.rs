use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(row: isize, col: isize) -> Self {
        Pos { x: row, y: col }
    }

    fn is_adjacent_to(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn apply_movement(&self, movement: &Movement) -> Self {
        match movement {
            Movement::Left => Self {
                x: self.x - 1,
                ..*self
            },
            Movement::Right => Self {
                x: self.x + 1,
                ..*self
            },
            Movement::Up => Self {
                y: self.y + 1,
                ..*self
            },
            Movement::Down => Self {
                y: self.y - 1,
                ..*self
            },
        }
    }
}

#[test]
fn test_is_adjacent_to() {
    assert!(Pos::new(3, 4).is_adjacent_to(&Pos::new(3, 4)));
    assert!(Pos::new(3, 4).is_adjacent_to(&Pos::new(3, 5)));

    assert!(!Pos::new(3, 4).is_adjacent_to(&Pos::new(3, 6)));
}

#[test]
fn test_apply_movement() {
    assert_eq!(
        Pos::new(0, 0).apply_movement(&Movement::Left),
        Pos::new(-1, 0)
    );

    assert_eq!(
        Pos::new(0, 0).apply_movement(&Movement::Right),
        Pos::new(1, 0)
    );
    assert_eq!(Pos::new(0, 0).apply_movement(&Movement::Up), Pos::new(0, 1));
    assert_eq!(
        Pos::new(0, 0).apply_movement(&Movement::Down),
        Pos::new(0, -1)
    );
}

#[derive(Debug, PartialEq)]
struct BoardState {
    knots: Vec<Pos>,
}

impl BoardState {
    fn new(n: usize) -> Self {
        Self {
            knots: (0..n).map(|_| Pos::new(0, 0)).collect::<Vec<Pos>>(),
        }
    }

    fn apply_movement(&mut self, movement: &Movement) {
        if self.knots.is_empty() {
            return;
        }

        let mut original = self.knots[0].clone();
        self.knots[0] = self.knots[0].apply_movement(movement);

        for i in 1..self.knots.len() {
            if !self.knots[i].is_adjacent_to(&self.knots[i - 1]) {
                (self.knots[i], original) = (original, self.knots[i].clone());
            } else {
                original = self.knots[i].clone();
            }
        }
    }
}

#[test]
fn test_apply_movement_board_right() {
    let mut board = BoardState::new(2);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    assert_eq!(
        board,
        BoardState {
            knots: vec![Pos { x: 4, y: 0 }, Pos { x: 3, y: 0 }],
        }
    );
}

#[test]
fn test_apply_movement_board_right_three_knots() {
    let mut board = BoardState::new(3);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    assert_eq!(
        board,
        BoardState {
            knots: vec![Pos { x: 4, y: 0 }, Pos { x: 3, y: 0 }, Pos { x: 2, y: 0 }],
        }
    );
}

#[test]
fn test_apply_movement_board_diagonal() {
    let mut board = BoardState::new(2);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Up);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Up);
    assert_eq!(
        board,
        BoardState {
            knots: vec![Pos { x: 2, y: 2 }, Pos { x: 1, y: 1 }],
        }
    );
}

fn watch_the_tail(movements: &[Movement], knot_size: usize) -> usize {
    let mut tail_visited = HashSet::new();
    let mut board = BoardState::new(knot_size);
    tail_visited.insert(board.knots.last().unwrap().clone());
    for m in movements {
        board.apply_movement(m);
        tail_visited.insert(board.knots.last().unwrap().clone());
    }
    tail_visited.len()
}

fn parse_movements(s: &str) -> Result<Vec<Movement>, Box<dyn Error>> {
    let mut movements = Vec::new();
    for line in s.lines() {
        if let [direction, count] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let cmd = match direction {
                "L" => Ok(Movement::Left),
                "R" => Ok(Movement::Right),
                "U" => Ok(Movement::Up),
                "D" => Ok(Movement::Down),
                _ => Err(format!("unknown direction: {}", direction)),
            }?;
            let count: i32 = count.parse()?;
            for _ in 0..count {
                movements.push(cmd.clone());
            }
        }
    }
    Ok(movements)
}

#[test]
fn test_watch_the_tail() -> Result<(), Box<dyn Error>> {
    assert_eq!(watch_the_tail(&parse_movements("R 1",)?, 3), 1);
    assert_eq!(watch_the_tail(&parse_movements("R 2",)?, 3), 1);
    assert_eq!(watch_the_tail(&parse_movements("R 3",)?, 3), 2);
    assert_eq!(
        watch_the_tail(&parse_movements("R 3\nL 1\nR 1\nL 1\n",)?, 3),
        2
    );
    Ok(())
}

#[test]
fn test_part_1_small_example() -> Result<(), Box<dyn Error>> {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(watch_the_tail(&parse_movements(input)?, 2), 13);

    Ok(())
}

#[test]
fn test_part_2_small_example() -> Result<(), Box<dyn Error>> {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(watch_the_tail(&parse_movements(input)?, 10), 1);

    Ok(())
}

#[test]
fn test_part_2_large_example_beginning_1() -> Result<(), Box<dyn Error>> {
    let input = "
R 5
";
    assert_eq!(watch_the_tail(&parse_movements(input,)?, 10,), 1);

    Ok(())
}

#[test]
fn test_part_2_large_example_beginning_2() -> Result<(), Box<dyn Error>> {
    let input = "
R 5
U 8
";
    assert_eq!(watch_the_tail(&parse_movements(input,)?, 10,), 1);

    Ok(())
}

#[test]
fn test_part_2_large_example() -> Result<(), Box<dyn Error>> {
    let input = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    assert_eq!(watch_the_tail(&parse_movements(input,)?, 10,), 36);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let movements = parse_movements(&std::fs::read_to_string(
        "adventofcode.com_2022_day_9_input.txt",
    )?)?;

    println!(
        "part 1: {} (should be 6181)",
        watch_the_tail(&movements[..], 2)
    );
    println!("part 2: {}", watch_the_tail(&movements[..], 10));
    Ok(())
}
