// https://adventofcode.com/2022/day/15

use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i32, i32);

impl Pos {
    // Returns Manhattan distance between two Pos.
    fn dist(self, other: Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, PartialEq)]
struct SensorData {
    sensor_at: Pos,
    beacon_at: Pos,
}

impl SensorData {
    fn beacon_radius(&self) -> u32 {
        self.sensor_at.dist(self.beacon_at)
    }

    // Returns a list of positions bounded by the marker and beacon.
    fn get_boundary(&self, y: i32) -> Vec<Pos> {
        let mut result = Vec::new();
        let beacon_radius = self.beacon_radius();
        let y_distance_from_sensor = self.sensor_at.1.abs_diff(y);
        if y_distance_from_sensor <= beacon_radius {
            let delta = (beacon_radius - y_distance_from_sensor) as i32;
            for i in -delta..=delta {
                result.push(Pos(self.sensor_at.0 + i as i32, y));
            }
        }

        result
    }
}

impl FromStr for SensorData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: OnceLock<Regex> = OnceLock::new();
        let pattern = PATTERN.get_or_init(|| {
            Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
                .unwrap()
        });
        let caps = pattern
            .captures(s)
            .ok_or_else(|| format!("could not parse {:?}", s))?;
        let make_error_fn = |loc| move |_| format!("could not parse {:?} in {}", s, loc);
        let [n1, n2, n3, n4] = [
            caps.get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .map_err(make_error_fn("sensor x"))?,
            caps.get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .map_err(make_error_fn("sensor y"))?,
            caps.get(3)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .map_err(make_error_fn("beacon x"))?,
            caps.get(4)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .map_err(make_error_fn("beacon y"))?,
        ];
        Ok(SensorData {
            sensor_at: Pos(n1, n2),
            beacon_at: Pos(n3, n4),
        })
    }
}

fn part_1(input: &str) -> usize {
    let all_sensor_data: Vec<SensorData> = input
        .lines()
        .map(SensorData::from_str)
        .collect::<Result<_, _>>()
        .expect("could not parse clean sensor data");
    let mut positions = HashSet::new();
    for data in &all_sensor_data {
        positions.extend(data.get_boundary(2000000));
    }
    for data in &all_sensor_data {
        positions.remove(&data.beacon_at);
    }

    positions.len()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");

    println!("part 1: {}", part_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
                .parse::<SensorData>()
                .unwrap(),
            SensorData {
                sensor_at: Pos(2, 18),
                beacon_at: Pos(-2, 15)
            }
        );
    }

    #[test]
    fn test_no_beacon_positions() {
        let s = SensorData {
            sensor_at: Pos(8, 7),
            beacon_at: Pos(2, 10),
        };
        assert_eq!(s.get_boundary(-3), vec![]);
        assert_eq!(s.get_boundary(-2), vec![Pos(8, -2)]);
        assert_eq!(s.get_boundary(-1), vec![Pos(7, -1), Pos(8, -1), Pos(9, -1)]);
    }

    const TEST_INPUT: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
