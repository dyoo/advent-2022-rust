use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug, PartialEq)]
struct Valve {
    id: String,
    flow_rate: i32,
    exits: Vec<String>,
}

impl std::str::FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: OnceLock<Regex> = OnceLock::new();

        let re: &Regex = RE.get_or_init(|| {
            let pattern = String::from(r"Valve (\w+) has flow rate=(\d+); ")
                + r"tunnels? leads? to valves? (\w+((, \w+)*))";
            Regex::new(&pattern).unwrap()
        });

        let captures = re
            .captures(s)
            .ok_or(format!("Could not parse {:?} as Valve", s))?;

        let id = captures.get(1).unwrap().as_str().to_owned();
        let flow_rate: i32 = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "Couldn't parse flow rate")?;
        let exits: Vec<String> = captures
            .get(3)
            .unwrap()
            .as_str()
            .split([' ', ','])
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        Ok(Valve {
            id,
            flow_rate,
            exits,
        })
    }
}

// It's a little easier if we enumerate the valves by number rather
// than string id.
#[derive(Debug, PartialEq)]
struct NormalizedValve {
    id: usize,
    name: String,
    flow_rate: i32,
    exits: Vec<usize>,
}

fn normalize_valves(valves: &[Valve]) -> Vec<NormalizedValve> {
    let mut mapping = HashMap::new();
    mapping.insert("AA", 0);
    let mut count: usize = 1;

    // Assign mappings for all ids.

    for id in valves
        .into_iter()
        .map(|valve| valve.id.as_str())
        .into_iter()
        .chain(
            valves
                .into_iter()
                .flat_map(|valve| valve.exits.iter().map(String::as_str)),
        )
    {
        if !mapping.contains_key(id) {
            mapping.insert(id, count);
            count += 1;
        }
    }

    let mut result: Vec<NormalizedValve> = valves
        .iter()
        .map(|valve| NormalizedValve {
            id: *mapping.get(valve.id.as_str()).expect("impossible"),
	    name: valve.id.clone(),
            flow_rate: valve.flow_rate,
            exits: valve
                .exits
                .iter()
                .map(|id| *mapping.get(id.as_str()).expect("impossible"))
                .collect(),
        })
        .collect();
    // Force AA to be at the top of the vector.
    result.sort_by_key(|valve| valve.id);
    result
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    at: usize,
    open: BTreeSet<usize>,
}

fn parse_valves(s: &str) -> Result<Vec<NormalizedValve>, String> {
    s.trim()
        .lines()
        .map(Valve::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map(|valves| normalize_valves(&valves[..]))
        .map_err(|e| e.to_string())
}

fn get_current_flow(open: &BTreeSet<usize>, valves: &[NormalizedValve]) -> i32 {
    open.iter().map(|id| valves[*id].flow_rate).sum()
}

fn get_optimal_total_flow(
    state: &State,
    valves: &[NormalizedValve],
    time_left: usize,
    cache: &mut HashMap<(State, usize), i32>,
) -> i32 {
    if time_left == 0 {
        return 0;
    }
    if let Some(v) = cache.get(&(state.clone(), time_left)) {
        return *v;
    }

    let current_flow = get_current_flow(&state.open, valves);
    let current_valve = &valves[state.at];

    // Available actions:
    //
    // * open valve (if closed)
    // * move to adjacent valve

    let score_after_opening = {
        // Opening the current valve:
        if !state.open.contains(&current_valve.id) && current_valve.flow_rate > 0 {
            let new_state = &State {
                at: state.at.clone(),
                open: {
                    let mut new_open = state.open.clone();
                    new_open.insert(current_valve.id);
                    new_open
                },
            };
            get_optimal_total_flow(new_state, valves, time_left - 1, cache)
        } else {
            // If we can't make this move, use zero to skip it
            0
        }
    };

    let scores_after_moving = current_valve.exits.iter().map(|exit| {
        let new_state = State {
            at: *exit,
            open: state.open.clone(),
        };
        get_optimal_total_flow(&new_state, valves, time_left - 1, cache)
    });

    let best_result =
        current_flow + std::cmp::max(scores_after_moving.max().unwrap_or(0), score_after_opening);

    cache.insert((state.clone(), time_left), best_result);

    best_result
}

fn part_1(s: &str) -> i32 {
    let valves = parse_valves(s).unwrap();
    let start_state = State {
        at: 0,
        open: BTreeSet::new(),
    };
    let cache = &mut HashMap::new();
    get_optimal_total_flow(&start_state, &valves, 30, cache)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("input.txt");
    println!("part 1: {}", part_1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "Valve AC has flow rate=4; tunnels lead to valves KC, RN, QA, QZ, UB";
        assert_eq!(
            s.parse::<Valve>(),
            Ok(Valve {
                id: "AC".into(),
                flow_rate: 4,
                exits: vec![
                    "KC".into(),
                    "RN".into(),
                    "QA".into(),
                    "QZ".into(),
                    "UB".into(),
                ]
            })
        );
    }

    #[test]
    fn test_parse_valves() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB
Valve BB has flow rate=13; tunnels lead to valves AA";
        assert_eq!(
            parse_valves(input).unwrap(),
            vec![
                NormalizedValve {
                    id: 0,
		    name: "AA".into(),
                    flow_rate: 0,
                    exits: vec![1],
                },
                NormalizedValve {
                    id: 1,
		    name: "BB".into(),
                    flow_rate: 13,
                    exits: vec![0],
                },
            ]
        );
    }

    #[test]
    fn test_get_current_flow_empty() {
        let input = "\
Valve AA has flow rate=5; tunnels lead to valves BB
Valve BB has flow rate=13; tunnels lead to valves CC";
        let valves = parse_valves(input).unwrap();
        assert_eq!(get_current_flow(&[].into(), &valves), 0);
    }

    #[test]
    fn test_get_current_flow_single() {
        let input = "\
Valve AA has flow rate=5; tunnels lead to valves BB
Valve BB has flow rate=13; tunnels lead to valves CC";
        let valves = parse_valves(input).unwrap();
        assert_eq!(get_current_flow(&[0].into(), &valves), 5);
    }

    #[test]
    fn test_get_current_flow_multiple() {
        let input = "\
Valve AA has flow rate=5; tunnels lead to valves BB
Valve BB has flow rate=13; tunnels lead to valves AA";
        let valves = parse_valves(input).unwrap();
        assert_eq!(get_current_flow(&[0, 1].into(), &valves), 18);
    }

    const SMALL_INPUT: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_get_optimal_total_flow() {
        let valves = parse_valves(SMALL_INPUT).unwrap();
        let start_state = State {
            at: 0,
            open: BTreeSet::new(),
        };
        let cache = &mut HashMap::new();
        assert_eq!(
            get_optimal_total_flow(&start_state, &valves, 30, cache),
            1651
        );
    }
}
