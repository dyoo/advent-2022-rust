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

fn main() {
    let pieces = [horiz(), plus(), corner(), vertical(), square()];
    for piece in pieces {
        println!("{:?}", piece.shift(2, 3));
    }
}
