use std::error::Error;

fn find_marker_end(s: &str, len: usize) -> Option<usize> {
    for (index, window) in s.as_bytes().windows(len).enumerate() {
        if is_all_different(window) {
            return Some(index + len);
        }
    }
    None
}

fn is_all_different(chars: &[u8]) -> bool {
    let mut seen = std::collections::HashSet::new();
    for &ch in chars {
        if seen.contains(&ch) {
            return false;
        }
        seen.insert(ch);
    }
    true
}

#[test]
fn test_start_of_packet() {
    assert_eq!(
        find_marker_end("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
        Some(7)
    );
    assert_eq!(find_marker_end("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    assert_eq!(find_marker_end("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    assert_eq!(
        find_marker_end("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
        Some(10)
    );
    assert_eq!(
        find_marker_end("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
        Some(11)
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("adventofcode.com_2022_day_6_input.txt")?;
    // Part 1:
    println!("{:?}", find_marker_end(&input, 4));

    // Part 2:
    println!("{:?}", find_marker_end(&input, 14));
    Ok(())
}
