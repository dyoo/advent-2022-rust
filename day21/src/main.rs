use day21::{find_minimum, parse_all_jobs};

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    let joblist = parse_all_jobs(&input);
    println!("part 1: {}", joblist.get_money("root"));

    let min = find_minimum(|x| joblist.loss(x), 0.1);
    println!("part 2: {}", min);

    println!(
        "part 2 guess 3342154812537: loss={}",
        joblist.loss(3342154812537.0)
    );
}
