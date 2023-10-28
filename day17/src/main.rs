use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn shift(self, x: i32, y: i32) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Piece {
    pos: Vec<Pos>,
}

impl Piece {
    fn shift(&self, x: i32, y: i32) -> Self {
        Piece {
            pos: self.pos.iter().copied().map(|p| p.shift(x, y)).collect(),
        }
    }

    fn left(&self) -> Self {
        self.shift(-1, 0)
    }

    fn right(&self) -> Self {
        self.shift(1, 0)
    }

    fn down(&self) -> Self {
        self.shift(0, 1)
    }
}

fn horiz() -> Piece {
    Piece {
        pos: vec![
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0),
            Pos::new(3, 0),
        ],
    }
}

fn plus() -> Piece {
    Piece {
        pos: vec![
            Pos::new(1, 0),
            Pos::new(0, 1),
            Pos::new(1, 1),
            Pos::new(2, 1),
            Pos::new(1, 2),
        ],
    }
}

fn corner() -> Piece {
    Piece {
        pos: vec![
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0),
            Pos::new(2, 1),
            Pos::new(2, 2),
        ],
    }
}
fn vertical() -> Piece {
    Piece {
        pos: vec![
            Pos::new(0, 0),
            Pos::new(0, 1),
            Pos::new(0, 2),
            Pos::new(0, 3),
        ],
    }
}

fn square() -> Piece {
    Piece {
        pos: vec![
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 1 },
        ],
    }
}

#[derive(Debug)]
struct Stage {
    filled: HashSet<Pos>,

    // the highest y that has a filled piece.  -1 at the very beginning which simulates the floor.
    top_y: i32,
}

impl Stage {
    fn new() -> Self {
        Self {
            filled: HashSet::new(),
            top_y: -1,
        }
    }

    fn add(&mut self, piece: &Piece) {
        self.filled.extend(piece.pos.iter());
        self.top_y = max(self.top_y, piece.pos.iter().map(|p| p.y).max().unwrap_or(0))
    }
}

// Returns true if any block in the piece collides with the stage or its boundnaries.
fn is_colliding(piece: &Piece, stage: &Stage) -> bool {
    piece
        .pos
        .iter()
        .any(|p| stage.filled.contains(p) || p.x < 0 || p.y < 0 || p.x >= 7)
}

fn place_initial(p: &Piece, stage: &Stage) -> Piece {
    p.shift(2, stage.top_y + 3)
}

fn part_1(input: &str) {
    // pieces will rotate among the following:
    let pieces = [horiz(), plus(), corner(), vertical(), square()]
        .into_iter()
        .cycle();

    // the instructions, similarly, will rotate:
    let repeating_instructions = input.trim().chars().cycle();

    let mut state = Stage::new();

    for instruction in repeating_instructions.clone().take(10) {
        dbg![instruction];
    }

    for piece in pieces.clone().take(2) {
        println!("{:?}", piece.shift(2, 3));
    }

    for piece in pieces.take(2) {
        println!("{:?}", piece.shift(2, 3));
    }
}

fn main() {
    // the instructions, similarly, will rotate:
    let input = std::fs::read_to_string("input.txt").expect("file");

    part_1(&input);
}
