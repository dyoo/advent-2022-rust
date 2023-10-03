use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl TryFrom<u32> for Shape {
    type Error = String;

    fn try_from(v: u32) -> Result<Shape, Self::Error> {
        match v {
            0 => Ok(Shape::Rock),
            1 => Ok(Shape::Paper),
            2 => Ok(Shape::Scissors),
            _ => Err(format!("Could not convert {} into Shape", v)),
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        *self as u32 + 1
    }

    fn versus(self, other: Self) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if (self as u32 + 1) % 3 == other as u32 {
            Outcome::Loss
        } else {
            Outcome::Win
        }
    }

    fn force_rhs_outcome(self, outcome: Outcome) -> Shape {
        match outcome {
            Outcome::Win => ((self as u32 + 1) % 3).try_into().unwrap(),
            Outcome::Loss => ((self as u32 + 2) % 3).try_into().unwrap(),
            Outcome::Draw => self,
        }
    }
}

#[test]
fn test_versus() {
    assert_eq!(Shape::Rock.versus(Shape::Paper), Outcome::Loss);
    assert_eq!(Shape::Rock.versus(Shape::Scissors), Outcome::Win);
    assert_eq!(Shape::Rock.versus(Shape::Rock), Outcome::Draw);

    assert_eq!(Shape::Paper.versus(Shape::Rock), Outcome::Win);
    assert_eq!(Shape::Paper.versus(Shape::Scissors), Outcome::Loss);
    assert_eq!(Shape::Paper.versus(Shape::Paper), Outcome::Draw);

    assert_eq!(Shape::Scissors.versus(Shape::Rock), Outcome::Loss);
    assert_eq!(Shape::Scissors.versus(Shape::Paper), Outcome::Win);
    assert_eq!(Shape::Scissors.versus(Shape::Scissors), Outcome::Draw);
}

#[test]
fn test_force_outcome() {
    assert_eq!(Shape::Rock.force_rhs_outcome(Outcome::Win), Shape::Paper);
    assert_eq!(
        Shape::Rock.force_rhs_outcome(Outcome::Loss),
        Shape::Scissors
    );

    assert_eq!(
        Shape::Paper.force_rhs_outcome(Outcome::Win),
        Shape::Scissors
    );
    assert_eq!(Shape::Paper.force_rhs_outcome(Outcome::Loss), Shape::Rock);

    assert_eq!(Shape::Scissors.force_rhs_outcome(Outcome::Win), Shape::Rock);
    assert_eq!(
        Shape::Scissors.force_rhs_outcome(Outcome::Loss),
        Shape::Paper
    );
}

fn parse_lhs(lhs: &str) -> Option<Shape> {
    match lhs {
        "A" => Some(Shape::Rock),
        "B" => Some(Shape::Paper),
        "C" => Some(Shape::Scissors),
        _ => None,
    }
}

fn parse_rhs_as_shape(lhs: &str) -> Option<Shape> {
    match lhs {
        "X" => Some(Shape::Rock),
        "Y" => Some(Shape::Paper),
        "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn parse_rhs_as_outcome(lhs: &str) -> Option<Outcome> {
    match lhs {
        "X" => Some(Outcome::Loss),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn score_round(lhs: Shape, rhs: Shape) -> u32 {
    match lhs.versus(rhs) {
        Outcome::Loss => 6 + rhs.score(),
        Outcome::Win => rhs.score(),
        Outcome::Draw => 3 + rhs.score(),
    }
}

fn total_score_part_1(to_read: impl Read) -> Result<u32, Box<dyn Error>> {
    let mut score = 0;
    for (lineindex, line) in BufReader::new(to_read).lines().enumerate() {
        let line = line?;
        let mut moves = line.split_ascii_whitespace();
        if let (Some(lhs), Some(rhs), None) = (moves.next(), moves.next(), moves.next()) {
            let (lhs, rhs): (Shape, Shape) = (
                parse_lhs(lhs)
                    .ok_or_else(|| format!("bad lhs on line {}: {}", lineindex + 1, line))?,
                parse_rhs_as_shape(rhs)
                    .ok_or_else(|| format!("bad rhs on line {}: {}", lineindex + 1, line))?,
            );
            score += score_round(lhs, rhs);
        }
    }
    Ok(score)
}

fn total_score_part_2(to_read: impl Read) -> Result<u32, Box<dyn Error>> {
    let mut score = 0;
    for (lineindex, line) in BufReader::new(to_read).lines().enumerate() {
        let line = line?;
        let mut moves = line.split_ascii_whitespace();
        if let (Some(lhs), Some(rhs), None) = (moves.next(), moves.next(), moves.next()) {
            let (lhs, rhs_outcome): (Shape, Outcome) = (
                parse_lhs(lhs)
                    .ok_or_else(|| format!("bad lhs on line {}: {}", lineindex + 1, line))?,
                parse_rhs_as_outcome(rhs)
                    .ok_or_else(|| format!("bad rhs on line {}: {}", lineindex + 1, line))?,
            );
            let rhs = lhs.force_rhs_outcome(rhs_outcome);
            score += score_round(lhs, rhs);
        }
    }
    Ok(score)
}

#[test]
fn test_total_score_main_example() -> Result<(), Box<dyn Error>> {
    assert_eq!(total_score_part_1(&b"A Y\nB X\nC Z"[..])?, 15);
    Ok(())
}

#[test]
fn test_total_score_part_2() -> Result<(), Box<dyn Error>> {
    assert_eq!(total_score_part_2(&b"A Y\nB X\nC Z"[..])?, 12);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("adventofcode.com_2022_day_2_input.txt")?;
    let score = total_score_part_1(input)?;
    println!("part 1: {}", score);

    let input = File::open("adventofcode.com_2022_day_2_input.txt")?;
    let score = total_score_part_2(input)?;
    println!("part 2: {}", score);

    Ok(())
}
