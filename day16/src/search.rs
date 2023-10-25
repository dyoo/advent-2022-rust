// Taking a search approach to the problem.  Trying to apply A*
//

use crate::*;
use std::cmp::max;
use std::collections::BinaryHeap;

fn all_pairs_shortest(valves: &[NormalizedValve]) -> Vec<Vec<u32>> {
    let n = valves.len();

    let mut costs = vec![vec![u32::MAX; n]; n];
    // Initial distances
    for (i, valve) in valves.iter().enumerate() {
        for exit in &valve.exits {
            costs[i][*exit] = 1;
        }
    }

    floyd_warshall(costs)
}

fn floyd_warshall(mut costs: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let n = costs.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if costs[i][k].saturating_add(costs[k][j]) < costs[i][j] {
                    costs[i][j] = costs[i][k].saturating_add(costs[k][j])
                }
            }
        }
    }
    costs
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerState {
    At { id: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    player_state: PlayerState,

    // Set of valves that are currently open.
    open: BitSet,

    // Total amount of flow so far.
    total_flow: u32,

    // Estimated total flow.
    estimated_total_flow: u32,

    // How much time is left.
    time_left: u32,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.estimated_total_flow.cmp(&other.estimated_total_flow)
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Returns list of closed valves.
fn get_closed_valves<'a>(open: &BitSet, valves: &'a [NormalizedValve]) -> Vec<&'a NormalizedValve> {
    let mut closed_valves = valves
        .iter()
        .filter(|v| !open.contains(v.id))
        .collect::<Vec<_>>();
    closed_valves.sort_by_key(|v| v.flow_rate);
    closed_valves
}

// Provides a optimistic best-case estimate for maximizing total flow
// at the end.  We make the simplifying assumptions that, of the
// unopened valves, they are all adjacent, and that we can open each
// in turn, from the largest to the smallest flow.
fn estimated_total_flow_heuristic(
    open: &BitSet,
    mut time_left: u32,
    valves: &[NormalizedValve],
) -> u32 {
    let mut total_flow = 0;

    if time_left < 2 {
        return total_flow;
    }

    let mut closed_valves = get_closed_valves(open, valves);

    // Now simulate opening each of the closed valves, one by one, and
    // acumulate flow.
    let mut open = open.clone();
    while time_left > 0 {
        // Open the next valve, in descending flow rate.
        if let Some(valve) = closed_valves.pop() {
            open.insert(valve.id);
            time_left -= 1;
        }

        if time_left > 0 {
            total_flow += get_current_flow(&open, valves);
            time_left -= 1;
        }
    }

    total_flow
}

pub fn find_optimal_total_flow(
    starting_at: usize,
    valves: &[NormalizedValve],
    time_left: u32,
) -> u32 {
    let distances = all_pairs_shortest(valves);

    let mut state_priority_queue = BinaryHeap::<State>::new();
    state_priority_queue.push(State {
        player_state: PlayerState::At { id: starting_at },
        open: BitSet::new(),
        total_flow: 0,
        time_left,
        estimated_total_flow: estimated_total_flow_heuristic(&BitSet::new(), time_left, valves),
    });
    drop(starting_at);
    drop(time_left);

    let mut best_solution_so_far = u32::MIN;

    while let Some(state) = state_priority_queue.pop() {
        let current_flow = get_current_flow(&state.open, valves);

        let PlayerState::At { id: player_at } = state.player_state;
        let distance_to = &distances[player_at];

        let accessible_closed_valves: Vec<&NormalizedValve> =
            get_closed_valves(&state.open, valves)
                .into_iter()
                .filter(|valve| distance_to[valve.id] < state.time_left)
                .collect();

        if accessible_closed_valves.is_empty() {
            best_solution_so_far = max(
                best_solution_so_far,
                state.total_flow + state.time_left * current_flow,
            );
        }

        // Visit a closed valve and open it, adding to priority queue unless
        // it has no chance of beating.
        for valve in accessible_closed_valves {}
    }

    best_solution_so_far
}

#[test]
fn test_floyd_warshall() {
    let inf = u32::MAX;

    //
    // x <----> y <-----> z
    //
    let input = vec![vec![0, 1, inf], vec![1, 0, 1], vec![inf, 1, 0]];
    let output = floyd_warshall(input);

    assert_eq!(output, vec![vec![0, 1, 2], vec![1, 0, 1], vec![2, 1, 0],]);
}
