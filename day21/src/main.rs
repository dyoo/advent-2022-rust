use day21::parse_all_jobs;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    let joblist = parse_all_jobs(&input);
    println!("part 1: {}", joblist.get_money("root"));
}
