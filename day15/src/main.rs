// https://adventofcode.com/2022/day/15

use range_set_blaze::RangeSetBlaze;
use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i32, i32);

impl Pos {
    // Returns Manhattan distance between two Pos.
    fn dist(self, other: Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn signal_strength(self) -> u64 {
        self.0 as u64 * 4000000 + self.1 as u64
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

    // Returns a bound of positions bounded by the marker and beacon on line y.
    fn get_boundary(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let beacon_radius = self.beacon_radius();
        let y_distance_from_sensor = self.sensor_at.1.abs_diff(y);
        if y_distance_from_sensor <= beacon_radius {
            let delta = (beacon_radius - y_distance_from_sensor) as i32;
            let (left, right) = ((self.sensor_at.0 - delta), (self.sensor_at.0 + delta));
            Some(left..=right)
        } else {
            None
        }
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

fn part_1(input: &str, y: i32) -> usize {
    let all_sensor_data: Vec<SensorData> = input
        .lines()
        .map(SensorData::from_str)
        .collect::<Result<_, _>>()
        .expect("could not parse clean sensor data");

    let mut positions = RangeSetBlaze::new();
    for data in &all_sensor_data {
        positions.extend(data.get_boundary(y));
    }

    for data in &all_sensor_data {
        if data.beacon_at.1 == y {
            positions.remove(data.beacon_at.0);
        }
    }

    positions.len()
}

fn find_distress_beacon(
    sensor_data: &Vec<SensorData>,
    x_bounds: i32,
    y_bounds: i32,
) -> Option<Pos> {
    let x_range = RangeSetBlaze::from_iter([0..=x_bounds]);

    for y in 0..=y_bounds {
        let mut positions = RangeSetBlaze::new();
        for data in sensor_data {
            positions.extend(data.get_boundary(y));
        }

        if !x_range.is_subset(&positions) {
            return (x_range - positions).first().map(|x| Pos(x, y));
        }
    }

    None
}

fn part_2(input: &str, x_bounds: i32, y_bounds: i32) -> Option<u64> {
    let all_sensor_data: Vec<SensorData> = input
        .lines()
        .map(SensorData::from_str)
        .collect::<Result<_, _>>()
        .expect("could not parse clean sensor data");
    find_distress_beacon(&all_sensor_data, x_bounds, y_bounds).map(Pos::signal_strength)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");

    println!("part 1: {:?}", part_1(&input, 2000000));
    println!("part 2: {:?}", part_2(&input, 4000000, 4000000));
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
        assert_eq!(s.get_boundary(-3), None);
        assert_eq!(s.get_boundary(-2), Some(8..=8));
        assert_eq!(s.get_boundary(-1), Some(7..=9));
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

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn test_find_distress_beacon() {
        let sensor_data: Vec<SensorData> = TEST_INPUT
            .lines()
            .map(SensorData::from_str)
            .collect::<Result<_, _>>()
            .expect("could not parse clean sensor data");
        assert_eq!(
            find_distress_beacon(&sensor_data, 20, 20),
            Some(Pos(14, 11))
        );
    }
}
