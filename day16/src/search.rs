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
    opened_valves: BitSet,

    // Total amount of flow so far.
    total_flow: u32,

    // Estimated total flow.
    estimated_total_flow: u32,

    // How much time is left.
    time_left: u32,
}

impl State {
    fn update_estimated_total_flow(mut self, valves: &[NormalizedValve]) -> Self {
        self.estimated_total_flow =
            self.total_flow + estimated_flow_heuristic(&self.opened_valves, self.time_left, valves);
        self
    }
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
fn get_closed_valves<'a>(
    opened: &BitSet,
    valves: &'a [NormalizedValve],
) -> Vec<&'a NormalizedValve> {
    let mut closed_valves = valves
        .iter()
        .filter(|v| !opened.contains(v.id))
        .collect::<Vec<_>>();
    closed_valves.sort_by_key(|v| v.flow_rate);
    closed_valves
}

// Provides a optimistic best-case estimate for maximizing total flow
// at the end.  We make the simplifying assumptions that, of the
// unopened valves, they are all adjacent, and that we can open each
// in turn, from the largest to the smallest flow.
fn estimated_flow_heuristic(opened: &BitSet, time_left: u32, valves: &[NormalizedValve]) -> u32 {
    let mut total_flow = 0;

    let mut closed_valves = get_closed_valves(opened, valves);

    // Now simulate opening each of the closed valves, one by one, and
    // acumulate flow.
    let mut opened = opened.clone();
    for i in 0..time_left {
        let current_flow = get_current_flow(&opened, valves);
        total_flow += current_flow;

        // Open the next valve, in descending flow rate, every other
        // tick, pretending that the player can teleport.
        if i % 2 == 0 {
            if let Some(valve) = closed_valves.pop() {
                opened.insert(valve.id);
            } else {
                // All valves are open: accelerate the rest of
                // the calculation.
                total_flow += current_flow * (time_left - i - 1);
                break;
            }
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
        opened_valves: BitSet::new(),
        total_flow: 0,
        time_left,
        estimated_total_flow: u32::MAX,
    });

    let mut best_solution_so_far = u32::MIN;

    while let Some(state) = state_priority_queue.pop() {
        let current_flow = get_current_flow(&state.opened_valves, valves);

        // Update the best solution by pretending we stay where we are
        // for the remainder of our time.
        best_solution_so_far = max(
            best_solution_so_far,
            state.total_flow + state.time_left * current_flow,
        );

        let PlayerState::At { id: player_at } = state.player_state;
        let distance_to = &distances[player_at];

        let accessible_closed_valves: Vec<&NormalizedValve> =
            get_closed_valves(&state.opened_valves, valves)
                .into_iter()
                .filter(|valve| distance_to[valve.id] < state.time_left)
                .filter(|valve| valve.flow_rate > 0)
                .collect();

        // Visit a closed valve and open it, adding to priority queue unless
        // it has no chance of beating the best so far.
        for valve in accessible_closed_valves {
            let new_player_state = PlayerState::At { id: valve.id };

            let mut new_opened_valves = state.opened_valves.clone();
            new_opened_valves.insert(valve.id);

            // Once we move and open, we measure how much flow has passed
            let new_time_passed = distance_to[valve.id] + 1;
            let new_total_flow = state.total_flow + new_time_passed * current_flow;
            let new_time_left = state.time_left - new_time_passed;

            let new_state = State {
                player_state: new_player_state,
                opened_valves: new_opened_valves,
                total_flow: new_total_flow,
                time_left: new_time_left,
                estimated_total_flow: u32::MAX,
            }
            .update_estimated_total_flow(valves);

            if new_state.estimated_total_flow > best_solution_so_far {
                state_priority_queue.push(new_state);
            }
        }
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
