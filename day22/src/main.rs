#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn forward1(&self, Pos { x, y, dir }: Pos) -> Pos {
        let (mut new_x, mut new_y) = (x, y);
        loop {
            match dir {
                Dir::North => {
                    new_y = new_y.checked_add_signed(-1).unwrap_or(self.map.len() - 1);
                }
                Dir::West => {
                    new_x = new_x.checked_add_signed(-1).unwrap_or(self.map[y].len() - 1);
                }
                Dir::South => {
                    new_y = (new_y + 1) % self.map.len();
                }
                Dir::East => {
                    new_x = (new_x + 1) % self.map[y].len();
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
    let map = parse_map(chunks.next()?);
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
    let pos = get_final_pos(s);
    pos.password()
}

/** Given the problem, returns final position. */
fn get_final_pos(s: &str) -> Pos {
    let problem = parse_input(s).unwrap();
    let mut pos = problem.initial_pos().unwrap();
    for &a in &problem.moves {
        pos = problem.apply_move(pos, a);
    }
    pos
}

/** Given the problem, shows what the path looks like.  For debugging purposes. */
#[allow(dead_code)]
fn visualize(s: &str) {
    let problem = parse_input(s).unwrap();
    let mut pos = problem.initial_pos().unwrap();
    let mut all_pos = vec![pos];

    let moves = problem
        .moves
        .iter()
        .cloned()
        .flat_map(|action| match action {
            Action::Forward(i) => (0..i).map(|_| Action::Forward(1)).collect::<Vec<Action>>(),
            Action::Clock => vec![Action::Clock],
            Action::Counterclock => vec![Action::Counterclock],
        })
        .collect::<Vec<Action>>();

    for &a in &moves {
        pos = problem.apply_move(pos, a);
        all_pos.push(pos);
    }

    let mut map = problem.map.clone();

    for Pos { x, y, dir } in all_pos {
        map[y][x] = match dir {
            Dir::North => '^',
            Dir::East => '>',
            Dir::South => 'V',
            Dir::West => '<',
        }
    }
    for line in &map {
        for char in line {
            print!("{}", char);
        }
        println!();
    }
}

#[test]
fn test_part1() {
    let input = "\
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

#[test]
fn test_get_final_pos() {
    let pos = get_final_pos(
        "   ...

1",
    );
    assert_eq!(
        pos,
        Pos {
            x: 4,
            y: 0,
            dir: Dir::East
        }
    );

    let pos = get_final_pos(
        "   ...

2",
    );
    assert_eq!(
        pos,
        Pos {
            x: 5,
            y: 0,
            dir: Dir::East
        }
    );

    let pos = get_final_pos(
        "   ...

3",
    );
    assert_eq!(
        pos,
        Pos {
            x: 3,
            y: 0,
            dir: Dir::East
        }
    );
}

#[test]
fn test_get_final_pos_left() {
    let pos = get_final_pos(
        "   ...

RR1",
    );
    assert_eq!(
        pos,
        Pos {
            x: 5,
            y: 0,
            dir: Dir::West
        }
    );

    let pos = get_final_pos(
        "   ...

RR2",
    );
    assert_eq!(
        pos,
        Pos {
            x: 4,
            y: 0,
            dir: Dir::West
        }
    );

    let pos = get_final_pos(
        "   ...

RR3",
    );
    assert_eq!(
        pos,
        Pos {
            x: 3,
            y: 0,
            dir: Dir::West
        }
    );
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    visualize(&input);
    println!("Part 1: {}", part_1(&input));
}
