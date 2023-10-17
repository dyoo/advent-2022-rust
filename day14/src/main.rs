use logos::Logos;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

#[derive(Debug)]
struct Cave {
    cells: HashMap<Pos, Cell>,
    y_boundary: i32,
}

impl Cave {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            y_boundary: 0,
        }
    }

    fn at(&self, p: Pos) -> Cell {
        self.cells.get(&p).copied().unwrap_or(Cell::Empty)
    }

    fn add_wall(&mut self, p: Pos) {
        self.cells.insert(p, Cell::Wall);
        if p.1 >= self.y_boundary {
            self.y_boundary = p.1 + 1;
        }
    }

    fn fill_wall_line(&mut self, p1: Pos, p2: Pos) {
        self.add_wall(p1);

        self.add_wall(p2);

        if p1 == p2 {
            return;
        }

        match (p1, p2) {
            (Pos(x1, y1), Pos(x2, y2)) if x1 == x2 => {
                let delta = (y2 - y1) / (y2 - y1).abs();
                let mut y = y1;
                while y != y2 {
                    self.add_wall(Pos(x1, y));

                    y += delta;
                }
            }
            (Pos(x1, y1), Pos(x2, y2)) if y1 == y2 => {
                let delta = (x2 - x1) / (x2 - x1).abs();
                let mut x = x1;
                while x != x2 {
                    self.add_wall(Pos(x, y1));
                    x += delta;
                }
            }
            _ => {
                // Ignore diagonals
            }
        }
    }

    // Returns `true` if sand is at rest, `false` if it falls into the abyss.
    fn drop_sand(&mut self, mut p: Pos) -> bool {
        while p.1 != self.y_boundary {
            let lower_left = Pos(p.0 - 1, p.1 + 1);
            let down = Pos(p.0, p.1 + 1);
            let lower_right = Pos(p.0 + 1, p.1 + 1);
            match (self.at(lower_left), self.at(down), self.at(lower_right)) {
                (_, Cell::Empty, _) => {
                    p = down;
                }
                (Cell::Empty, _, _) => {
                    p = lower_left;
                }
                (_, _, Cell::Empty) => {
                    p = lower_right;
                }
                _ => {
                    break;
                }
            }
        }

        if p.1 != self.y_boundary {
            self.cells.insert(p, Cell::Sand);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Sand,
    Wall,
}

// Tokenizer for reading the input, the list of positions that form
// the walls.
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // ignore whitespace
enum Token {
    #[regex(r"\d+", |lex| lex.slice().parse().ok())]
    Num(i32),

    #[token(",")]
    Comma,

    #[token("->")]
    Arrow,
}

fn parse_line(s: &str) -> Vec<Pos> {
    let mut result = Vec::new();
    let mut lexer = Token::lexer(s);

    while let (Some(Ok(Token::Num(x))), Some(Ok(Token::Comma)), Some(Ok(Token::Num(y)))) =
        (lexer.next(), lexer.next(), lexer.next())
    {
        result.push(Pos(x, y));

        // Eat the arrow
        if let Some(Ok(Token::Arrow)) = lexer.next() {
        } else {
            break;
        }
    }

    result
}

fn part_1(input: &str) -> usize {
    let position_lists: Vec<Vec<Pos>> = input.lines().map(parse_line).collect();
    let mut cave = Cave::new();

    // Fill in the walls
    for positions in position_lists {
        for pair in positions.windows(2) {
            cave.fill_wall_line(pair[0], pair[1]);
        }
    }

    let mut i = 0;
    loop {
        if !cave.drop_sand(Pos(500, 0)) {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    println!("part 1: {}", part_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("484,41 -> 484,42 -> 495,42 -> 495,41"),
            vec![Pos(484, 41), Pos(484, 42), Pos(495, 42), Pos(495, 41)]
        );
    }

    #[test]
    fn test_fill_wall_down() {
        let mut cave = Cave::new();
        cave.fill_wall_line(Pos(0, 0), Pos(0, 3));
        assert_eq!(
            cave.cells,
            HashMap::from([
                (Pos(0, 0), Cell::Wall),
                (Pos(0, 1), Cell::Wall),
                (Pos(0, 2), Cell::Wall),
                (Pos(0, 3), Cell::Wall),
            ])
        );
        assert_eq!(cave.y_boundary, 4);
    }

    #[test]
    fn test_fill_wall_up() {
        let mut cave = Cave::new();
        cave.fill_wall_line(Pos(1, 3), Pos(1, 0));
        assert_eq!(
            cave.cells,
            HashMap::from([
                (Pos(1, 3), Cell::Wall),
                (Pos(1, 2), Cell::Wall),
                (Pos(1, 1), Cell::Wall),
                (Pos(1, 0), Cell::Wall),
            ])
        );
        assert_eq!(cave.y_boundary, 4);
    }

    #[test]
    fn test_fill_wall_left() {
        let mut cave = Cave::new();
        cave.fill_wall_line(Pos(2, 3), Pos(0, 3));
        assert_eq!(
            cave.cells,
            HashMap::from([
                (Pos(2, 3), Cell::Wall),
                (Pos(1, 3), Cell::Wall),
                (Pos(0, 3), Cell::Wall),
            ])
        );
        assert_eq!(cave.y_boundary, 4);
    }

    #[test]
    fn test_fill_wall_right() {
        let mut cave = Cave::new();
        cave.fill_wall_line(Pos(2, 3), Pos(0, 3));
        assert_eq!(
            cave.cells,
            HashMap::from([
                (Pos(2, 3), Cell::Wall),
                (Pos(1, 3), Cell::Wall),
                (Pos(0, 3), Cell::Wall),
            ])
        );
        assert_eq!(cave.y_boundary, 4);
    }

    #[test]
    fn test_fill_wall_same() {
        let mut cave = Cave::new();
        cave.fill_wall_line(Pos(2, 3), Pos(2, 3));
        assert_eq!(cave.cells, HashMap::from([(Pos(2, 3), Cell::Wall),]));
        assert_eq!(cave.y_boundary, 4);
    }

    #[test]
    fn test_at() {
        let mut cave = Cave::new();
        cave.fill_wall_line(Pos(2, 3), Pos(2, 3));
        assert_eq!(cave.at(Pos(2, 3)), Cell::Wall);
        assert_eq!(cave.at(Pos(2, 4)), Cell::Empty);
    }

    #[test]
    fn test_part1() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part_1(&input), 24);
    }
}
