use day10::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("adventofcode.com_2022_day_10_input.txt")?;

    println!("part 1: {}", part_1(&input));
    println!();
    println!("part 2:\n{}", part_2(&input));

    Ok(())
}
