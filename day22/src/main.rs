#[derive(Debug, Clone, Copy)]
enum Dir {
    North = 3,
    East = 0,
    South = 1,
    West = 2,
}
impl Dir {
    fn clock(self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn counterclock(self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
}
impl Pos {
    fn password(&self) -> i32 {
        1000 * (self.y as i32 + 1) + 4 * (self.x as i32 + 1) + self.dir as i32
    }
}

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<char>>,
    moves: Vec<Action>,
}

impl Problem {
    fn initial_pos(&self) -> Option<Pos> {
        for (i, ch) in self.map[0].iter().enumerate() {
            if *ch == '.' {
                return Some(Pos {
                    x: i,
                    y: 0,
                    dir: Dir::East,
                });
            }
        }
        None
    }

    fn forward1(&self, Pos { mut x, mut y, dir }: Pos) -> Pos {
        loop {
            let (mut new_x, mut new_y) = (x, y);
            match dir {
                Dir::North => {
                    new_y = y.checked_add_signed(-1).unwrap_or(self.map.len() - 1);
                }
                Dir::West => {
                    new_x = x.checked_add_signed(-1).unwrap_or(self.map[y].len() - 1);
                }
                Dir::South => {
                    new_y = (y + 1) % self.map.len();
                }
                Dir::East => {
                    new_x = (x + 1) % self.map[y].len();
                }
            }

            match self.map[new_y].get(new_x).unwrap_or(&' ') {
                '#' => {
                    // Hit a wall: stop moving
                    return Pos { x, y, dir };
                }
                '.' => {
                    // Landed in vacant spot
                    return Pos {
                        x: new_x,
                        y: new_y,
                        dir,
                    };
                }
                _ => {
                    // Out of bounds.  Keep moving.
                    x = new_x;
                    y = new_y;
                }
            }
        }
    }

    fn apply_move(&self, p: Pos, a: Action) -> Pos {
        match a {
            Action::Forward(n) => (0..n).fold(p, |acc, _| self.forward1(acc)),

            Action::Clock => Pos {
                dir: p.dir.clock(),
                ..p
            },

            Action::Counterclock => Pos {
                dir: p.dir.counterclock(),
                ..p
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
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

fn parse_moves(s: &str) -> Vec<Action> {
    let mut moves = Vec::new();
    let mut n = 0;
    for ch in s.trim().chars() {
        match ch {
            '0'..='9' => n = n * 10 + (ch as usize - '0' as usize),
            'L' => {
                moves.push(Action::Forward(n));
                moves.push(Action::Counterclock);
                n = 0;
            }
            'R' => {
                moves.push(Action::Forward(n));
                moves.push(Action::Clock);
                n = 0;
            }
            _ => {}
        }
    }
    if n != 0 {
        moves.push(Action::Forward(n));
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

fn part_1(s: &str) -> i32 {
    let problem = parse_input(s).unwrap();
    let mut pos = problem.initial_pos().unwrap();
    for &a in &problem.moves {
        pos = problem.apply_move(pos, a);
    }
    pos.password()
}

#[test]
fn test_part1() {
    let input = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
    assert_eq!(part_1(input), 6032);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
}
