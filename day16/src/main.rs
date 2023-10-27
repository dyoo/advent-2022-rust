fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    println!("part 1: {}", day16::part_1(&input));
    println!(
        "part 1 (with search): {}",
        day16::part_1_with_search(&input)
    );
    println!(
        "part 1 (with search): {}",
        day16::part_2_with_search(&input)
    );
}
