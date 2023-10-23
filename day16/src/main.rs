use day16::part_1;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    println!("part 1: {}", part_1(&input));
}
