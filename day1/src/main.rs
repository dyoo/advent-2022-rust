/// Solution for https://adventofcode.com/2022/day/1
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// Returns largest value.
fn read_max_sum(to_read: impl Read) -> Result<u32, Box<dyn Error>> {
    let sums = read_sums(to_read)?;
    Ok(sums.into_iter().max().unwrap_or(0))
}

/// Returns largest three values.
fn read_max_three(
    to_read: impl Read,
) -> Result<(Option<u32>, Option<u32>, Option<u32>), Box<dyn Error>> {
    let mut sums = read_sums(to_read)?;
    sums.sort();
    sums.reverse();
    let top_three = (
        sums.get(0).copied(),
        sums.get(1).copied(),
        sums.get(2).copied(),
    );
    Ok(top_three)
}

/// Returns all summed values.
fn read_sums(to_read: impl Read) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut sum = 0;
    let mut result = Vec::new();

    for line in BufReader::new(to_read).lines() {
        let line = line?;
        if line.is_empty() {
            result.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>()?;
        }
    }
    result.push(sum);
    Ok(result)
}

#[test]
fn test_read_data_single() -> Result<(), Box<dyn Error>> {
    assert_eq!(read_max_sum(r"1".as_bytes(),)?, 1);

    Ok(())
}

#[test]
fn test_read_data_empty() -> Result<(), Box<dyn Error>> {
    assert_eq!(read_max_sum("".as_bytes(),)?, 0);

    Ok(())
}

#[test]
fn test_read_data_sums() -> Result<(), Box<dyn Error>> {
    assert_eq!(read_max_sum("1\n2\n3".as_bytes(),)?, 6);

    Ok(())
}

#[test]
fn test_read_data_main_example() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        read_max_sum(
            r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#
                .as_bytes(),
        )?,
        24000
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("adventofcode.com_2022_day_1_input.txt")?;
    let max_value = read_max_sum(file)?;
    println!("{:?}", max_value);

    let file = File::open("adventofcode.com_2022_day_1_input.txt")?;
    let max_three_values = read_max_three(file)?;
    println!("{:?}", max_three_values);
    println!(
        "{:?}",
        max_three_values.0.ok_or("No item 1")?
            + max_three_values.1.ok_or("No item 2")?
            + max_three_values.2.ok_or("No item 3")?
    );

    Ok(())
}
