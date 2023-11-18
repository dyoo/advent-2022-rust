#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn clock(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn counterclock(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
enum Move {
    Forward(usize),
    Clock,
    Counterclock,
}

fn parse_input(s: &str) -> Option<Problem> {
    let mut chunks = s.split("\n\n");
    let map = parse_map(chunks.next()?.trim());
    let moves = parse_moves(chunks.next()?);

    Some(Problem { map, moves })
}

fn parse_moves(s: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut n = 0;
    for ch in s.trim().chars() {
        match ch {
            '0'..='9' => n = n * 10 + (ch as usize - '0' as usize),
            'L' => {
                moves.push(Move::Forward(n));
                moves.push(Move::Counterclock);
                n = 0;
            }
            'R' => {
                moves.push(Move::Forward(n));
                moves.push(Move::Clock);
                n = 0;
            }
            _ => {}
        }
    }
    if n != 0 {
        moves.push(Move::Forward(n));
    }
    moves
}

fn parse_map(s: &str) -> Vec<Vec<char>> {
    let map = s
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    map
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    dbg!(parse_input(&input).unwrap());
}
