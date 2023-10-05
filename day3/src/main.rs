use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn priority(ch: char) -> u32 {
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => todo!(),
    }
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('L'), 38);
}

fn get_sum_priorities_part_1(input: impl Read) -> u32 {
    let mut sum = 0;
    for line in BufReader::new(input).lines() {
        let sack: Vec<char> = line.unwrap().chars().collect();
        let (lhs, rhs) = sack.split_at(sack.len() / 2);

        let lhs_set: HashSet<char> = lhs.iter().copied().collect();
        let rhs_set: HashSet<char> = rhs.iter().copied().collect();

        for shared in lhs_set.intersection(&rhs_set).copied() {
            sum += priority(shared);
        }
    }
    sum
}

#[test]
fn test_sum_priorities_example() {
    let ex = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        .as_bytes();
    assert_eq!(get_sum_priorities_part_1(ex), 157);
}

fn get_sum_priorities_part_2(input: impl Read) -> u32 {
    let mut sum = 0;
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    for group in lines.chunks_exact(3) {
        let set1: HashSet<char> = group[0].chars().collect();
        let set2: HashSet<char> = group[1].chars().collect();
        let set3: HashSet<char> = group[2].chars().collect();

        for ch in set1 {
            if set2.contains(&ch) && set3.contains(&ch) {
                sum += priority(ch);
            }
        }
    }
    sum
}

#[test]
fn test_sum_priorities_2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        .as_bytes();
    assert_eq!(get_sum_priorities_part_2(input), 70);
}

fn main() {
    let f = File::open("adventofcode.com_2022_day_3_input.txt").unwrap();
    println!("{}", get_sum_priorities_part_1(f));

    let f = File::open("adventofcode.com_2022_day_3_input.txt").unwrap();
    println!("{}", get_sum_priorities_part_2(f));
}
