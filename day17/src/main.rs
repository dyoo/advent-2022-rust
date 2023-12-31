use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i64,
}

impl Pos {
    fn new(x: i32, y: i64) -> Self {
        Self { x, y }
    }

    fn shift(self, x: i32, y: i64) -> Self {
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
    fn shift(&self, x: i32, y: i64) -> Self {
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
        self.shift(0, -1)
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
    top_y: i64,
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
    p.shift(2, stage.top_y + 4)
}

fn height_after_blocks_fall(jet_pattern_input: &str, max_stones: i64) -> i64 {
    // pieces will rotate among the following:
    let mut pieces = [horiz(), plus(), corner(), vertical(), square()]
        .into_iter()
        .cycle()
        .into_iter();

    // the instructions, similarly, will rotate:
    let mut jets = jet_pattern_input.trim().chars().cycle().into_iter();

    let mut stage = Stage::new();

    let mut count = 0;
    let mut piece = place_initial(&pieces.next().unwrap(), &stage);

    loop {
        // Handle jet movement.
        let jet = jets.next().unwrap();
        let mut blown = piece.clone();
        if jet == '<' {
            blown = blown.left();
        } else if jet == '>' {
            blown = blown.right();
        }
        if !is_colliding(&blown, &stage) {
            piece = blown;
        }

        // Handle falling.
        let fallen = piece.clone().down();
        if is_colliding(&fallen, &stage) {
            stage.add(&piece);
            count += 1;

            piece = place_initial(&pieces.next().unwrap(), &stage);
        } else {
            piece = fallen;
        }

        if count >= max_stones {
            break;
        }
    }

    stage.top_y + 1
}

fn main() {
    // the instructions, similarly, will rotate:
    let input = std::fs::read_to_string("input.txt").expect("file");

    println!("part 1: {}", height_after_blocks_fall(&input, 2022));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_initial_empty() {
        let p = place_initial(&horiz(), &Stage::new());
        assert_eq!(
            p.pos,
            vec![
                Pos::new(2, 3),
                Pos::new(3, 3),
                Pos::new(4, 3),
                Pos::new(5, 3)
            ]
        );
    }

    #[test]
    fn test_place_initial_after_horiz_on_floor() {
        let mut stage = Stage::new();
        stage.add(&horiz());
        let p = place_initial(&plus(), &stage);
        assert_eq!(
            p.pos,
            vec![
                Pos::new(3, 4),
                Pos::new(2, 5),
                Pos::new(3, 5),
                Pos::new(4, 5),
                Pos::new(3, 6),
            ]
        );
    }

    #[test]
    fn test_is_colliding() {
        let stage = Stage::new();

        let piece = horiz();
        assert!(!is_colliding(&piece, &stage));

        let piece = horiz().down();
        assert!(is_colliding(&piece, &stage));

        let piece = horiz().shift(3, 0);
        assert!(!is_colliding(&piece, &stage));

        let piece = horiz().shift(4, 0);
        assert!(is_colliding(&piece, &stage));

        let piece = horiz().shift(-1, 0);
        assert!(is_colliding(&piece, &stage));
    }

    #[test]
    fn test_part_1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(height_after_blocks_fall(input, 2022), 3068);
    }

    #[test]
    fn test_part_2() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(
            height_after_blocks_fall(input, 1000000000000),
            1514285714288
        );
    }
}
