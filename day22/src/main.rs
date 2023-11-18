#[derive(Debug)]
struct Problem<'a> {
    map: Vec<Vec<char>>,
    moves: &'a str,
}

fn parse_input(s: &str) -> Option<Problem> {
    let mut chunks = s.split("\n\n");
    let map = chunks
        .next()?
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let moves = chunks.next()?.trim();
    Some(Problem { map, moves })
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    dbg!(parse_input(&input).unwrap());
}
