use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
struct State {
    columns: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let chunks: Vec<&str> = s.split_whitespace().collect();
        match chunks[..] {
            ["move", how_many, "from", from, "to", to] => Ok(Move {
                how_many: how_many
                    .parse()
                    .map_err(|x| format!("could not parse 'how_many': {}", x))?,
                from: from
                    .parse::<usize>()
                    .map_err(|x| format!("could not parse 'from': {}", x))?
                    - 1,
                to: to
                    .parse::<usize>()
                    .map_err(|x| format!("could not parse 'to': {}", x))?
                    - 1,
            }),
            _ => Err(format!("could not parse Move from: {:?}", s)),
        }
    }
}

#[test]
fn parse_move() {
    assert_eq!(
        "move 3 from 8 to 9".parse::<Move>(),
        Ok(Move {
            how_many: 3,
            from: 8,
            to: 9
        })
    );
}

fn apply_move_part_1(
    state: &mut State,
    &Move { how_many, from, to }: &Move,
) -> Result<(), Box<dyn Error>> {
    for _ in 0..how_many {
        let elt = state.columns[from]
            .pop()
            .ok_or(format!("not enough columns in {}", from))?;
        state.columns[to].push(elt);
    }
    Ok(())
}

fn apply_move_part_2(
    state: &mut State,
    &Move { how_many, from, to }: &Move,
) -> Result<(), Box<dyn Error>> {
    let mut to_move = Vec::new();
    for _ in 0..how_many {
        to_move.push(
            state.columns[from]
                .pop()
                .ok_or(format!("not enough columns in {}", from))?,
        );
    }
    to_move.reverse();
    state.columns[to].extend(to_move);

    Ok(())
}

fn parse_columns(input: &mut impl BufRead) -> Result<State, Box<dyn Error>> {
    let mut saved_lines = Vec::<String>::new();
    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        saved_lines.push(line);
    }
    let column_number_line = saved_lines.last().ok_or("Missing last line?")?;
    let mut indices: Vec<usize> = Vec::new();
    for (index, char) in column_number_line.chars().enumerate() {
        if char.is_numeric() {
            indices.push(index);
        }
    }

    let mut columns: Vec<Vec<char>> = vec![vec![]; indices.len()];

    for line in saved_lines[..(saved_lines.len() - 1)].into_iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for (column, &index) in indices.iter().enumerate() {
            if let Some(ch) = chars.get(index) {
                if ch.is_ascii_alphabetic() {
                    columns[column].push(*ch);
                }
            }
        }
    }

    Ok(State { columns })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Part 1
    let mut input = BufReader::new(File::open("adventofcode.com_2022_day_5_input.txt")?);
    let mut state = parse_columns(&mut input)?;
    println!("{:?}", state);
    for line in input.lines() {
        let next_move = line?.parse::<Move>()?;
        apply_move_part_1(&mut state, &next_move)?;
    }

    for column in state.columns {
        match column.last().ok_or("empty") {
            Ok(last) => {
                print!("{}", last)
            }
            Err(_) => {
                println!("unexpected empty!");
            }
        }
    }
    println!();

    // Part 2
    let mut input = BufReader::new(File::open("adventofcode.com_2022_day_5_input.txt")?);
    let mut state = parse_columns(&mut input)?;
    println!("{:?}", state);
    for line in input.lines() {
        let next_move = line?.parse::<Move>()?;
        apply_move_part_2(&mut state, &next_move)?;
    }

    for column in state.columns {
        match column.last().ok_or("empty") {
            Ok(last) => {
                print!("{}", last)
            }
            Err(_) => {
                println!("unexpected empty!");
            }
        }
    }
    println!();

    Ok(())
}
