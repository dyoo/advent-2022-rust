use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Assignment {
        Assignment { start, end }
    }

    fn fully_encloses(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    // https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl std::str::FromStr for Assignment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split("-");
        if let (Some(start), Some(end), None) = (chunks.next(), chunks.next(), chunks.next()) {
            let start = start.parse::<u32>();
            let end = end.parse::<u32>();
            if let (Ok(start), Ok(end)) = (start, end) {
                return Ok(Assignment::new(start, end));
            }
        }

        Err(format!("Couldn't parse {}", s))
    }
}

#[test]
fn test_fully_encloses() {
    assert!(Assignment::new(1, 2).fully_encloses(&Assignment::new(3, 4)) == false);

    assert!(Assignment::new(1, 2).fully_encloses(&Assignment::new(2, 2)));

    assert!(Assignment::new(2, 8).fully_encloses(&Assignment::new(3, 7)));
}

#[test]
fn test_parse() {
    assert!(matches!("hello.".parse::<Assignment>(), Err(_)));
    assert!(matches!(
        "2-4".parse::<Assignment>(),
        Ok(Assignment { start: 2, end: 4 })
    ));
    assert!(matches!(
        "12-17".parse::<Assignment>(),
        Ok(Assignment { start: 12, end: 17 })
    ));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Part 1
    let mut overlap_count = 0;
    for line in BufReader::new(File::open("adventofcode.com_2022_day_4_input.txt")?).lines() {
        let line = line?;
        let mut chunks = line.split(",");
        if let (Some(x), Some(y)) = (chunks.next(), chunks.next()) {
            let x = x.parse::<Assignment>()?;
            let y = y.parse::<Assignment>()?;
            if x.fully_encloses(&y) || y.fully_encloses(&x) {
                overlap_count += 1;
            }
        }
    }

    println!("{}", overlap_count);

    // Part 2
    let mut overlap_count = 0;
    for line in BufReader::new(File::open("adventofcode.com_2022_day_4_input.txt")?).lines() {
        let line = line?;
        let mut chunks = line.split(",");
        if let (Some(x), Some(y)) = (chunks.next(), chunks.next()) {
            let x = x.parse::<Assignment>()?;
            let y = y.parse::<Assignment>()?;
            if x.overlaps(&y) {
                overlap_count += 1;
            }
        }
    }

    println!("{}", overlap_count);

    Ok(())
}
