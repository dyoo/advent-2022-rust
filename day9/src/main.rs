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

    assert!(Pos::new(3, 4).is_adjacent_to(&Pos::new(3, 6)) == false);
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
    head: Pos,
    tail: Pos,
}

impl BoardState {
    fn new() -> Self {
        Self {
            head: Pos::new(0, 0),
            tail: Pos::new(0, 0),
        }
    }

    fn apply_movement(&mut self, movement: &Movement) {
        let new_head = self.head.apply_movement(movement);
        if new_head.is_adjacent_to(&self.tail) {
            self.head = new_head;
        } else {
            self.tail = self.head.clone();
            self.head = new_head;
        }
    }
}

#[test]
fn test_apply_movement_board_right() {
    let mut board = BoardState::new();
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Right);
    assert_eq!(
        board,
        BoardState {
            head: Pos { x: 4, y: 0 },
            tail: Pos { x: 3, y: 0 }
        }
    );
}

#[test]
fn test_apply_movement_board_diagonal() {
    let mut board = BoardState::new();
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Up);
    board.apply_movement(&Movement::Right);
    board.apply_movement(&Movement::Up);
    assert_eq!(
        board,
        BoardState {
            head: Pos { x: 2, y: 2 },
            tail: Pos { x: 1, y: 1 }
        }
    );
}

fn part_1(movements: &[Movement]) -> usize {
    let mut tail_visited = HashSet::new();
    let mut board = BoardState::new();
    tail_visited.insert(board.tail.clone());
    for m in movements {
        board.apply_movement(m);
        tail_visited.insert(board.tail.clone());
    }
    tail_visited.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut movements = Vec::new();
    for line in std::fs::read_to_string("adventofcode.com_2022_day_9_input.txt")?.lines() {
        if let [direction, count] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let cmd = match direction {
                "L" => Movement::Left,
                "R" => Movement::Right,
                "U" => Movement::Up,
                "D" => Movement::Down,
                _ => panic!(),
            };
            let count: i32 = count.parse()?;
            for _ in 0..count {
                movements.push(cmd.clone());
            }
        }
    }
    println!("{}", part_1(&movements[..]));

    Ok(())
}
