// Taking a search approach to the problem.  Trying to apply A*
//

use crate::*;
use std::cmp::{max, min};
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PlayerState {
    Wait { at: usize, time_left: u32 },
    Travel { to: usize, time_left: u32 },
    Open { at: usize, time_left: u32 },
}

const TIME_TO_OPEN_VALVE: u32 = 1;

// Intended use of PlayerState:
//
// * Use tick to find minimum time of stability (for flow accumulation)
// * Use apply to apply changes the player makes to the state.
// * Use nexts to find new actions for each player.
impl PlayerState {
    fn get_time_till_action(&self) -> u32 {
        match self {
            PlayerState::Wait { time_left, .. } => *time_left,
            PlayerState::Travel { time_left, .. } => *time_left,
            PlayerState::Open { time_left, .. } => *time_left,
        }
    }

    fn destination(&self) -> usize {
        match self {
            PlayerState::Wait { at: id, .. } => *id,
            PlayerState::Travel { to: id, .. } => *id,
            PlayerState::Open { at: id, .. } => *id,
        }
    }

    fn tick(&mut self, time_passed: u32) {
        match self {
            PlayerState::Wait { time_left, .. } => {
                *time_left = time_left.saturating_sub(time_passed);
            }
            PlayerState::Travel { time_left, .. } => {
                *time_left = time_left.saturating_sub(time_passed);
            }
            PlayerState::Open { time_left, .. } => {
                *time_left = time_left.saturating_sub(time_passed);
            }
        }
    }

    // Returns list of new player states.
    fn get_next_states(
        self,
        state: &State,
        valves: &[NormalizedValve],
        distances: &[Vec<u32>],
    ) -> Vec<PlayerState> {
        match self {
            PlayerState::Wait { at, time_left } => {
                if time_left != 0 {
                    return vec![self];
                }

                // Schedule a visit to a closed valve that has flow.
                let distance_to = &distances[at];

                // TODO: handle multiplayer
                let other_player_destinations = Some(&state.player_state)
                    .into_iter()
                    .map(PlayerState::destination)
                    .collect::<BitSet>();

                let accessible_closed_valves: Vec<&NormalizedValve> =
                    get_closed_valves(&state.opened_valves, valves)
                        .into_iter()
                        .filter(|valve| distance_to[valve.id] < state.time_left)
                        .filter(|valve| valve.flow_rate > 0)
                        .filter(|valve| !other_player_destinations.contains(valve.id))
                        .collect();

                let results: Vec<PlayerState> = accessible_closed_valves
                    .into_iter()
                    .map(|valve| PlayerState::Travel {
                        to: valve.id,
                        time_left: distance_to[valve.id],
                    })
                    .collect();

                if results.is_empty() {
                    vec![PlayerState::Wait {
                        at,
                        time_left: state.time_left,
                    }]
                } else {
                    results
                }
            }
            PlayerState::Travel { to: at, time_left } => {
                if time_left == 0 {
                    vec![PlayerState::Open {
                        at,
                        time_left: TIME_TO_OPEN_VALVE,
                    }]
                } else {
                    vec![self]
                }
            }
            PlayerState::Open { at, time_left } => {
                if time_left == 0 {
                    vec![PlayerState::Wait {
                        at: at,
                        time_left: 0,
                    }]
                } else {
                    vec![self]
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct State {
    // What each player is doing.  At the moment, just one player.
    player_state: PlayerState,

    // Set of valves that are open.
    opened_valves: BitSet,

    // Total amount of flow so far.
    accumulated_flow: u32,

    // How much time is left.
    time_left: u32,

    // Estimated total flow once time expires.
    estimated_total_flow: u32,
}

// Expected transitions:
//
// tick() ------> apply_player_actions() -----> get_next_states()

impl State {
    // Provides a optimistic best-case estimate for maximizing total flow
    // at the end.  We make the simplifying assumptions that, of the
    // unopened valves, they are all adjacent, and that we can open each
    // in turn, from the largest to the smallest flow.
    fn estimated_flow_heuristic(&self, valves: &[NormalizedValve]) -> u32 {
        let mut total_flow = 0;

        let mut closed_valves = get_closed_valves(&self.opened_valves, valves);

        // Now simulate opening each of the closed valves, one by one, and
        // acumulate flow.
        let mut opened = self.opened_valves.clone();
        for i in 0..self.time_left {
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
                    return total_flow + current_flow * (self.time_left - i - 1);
                }
            }
        }

        total_flow
    }

    fn update_estimated_total_flow(mut self, valves: &[NormalizedValve]) -> Self {
        self.estimated_total_flow = self.accumulated_flow + self.estimated_flow_heuristic(valves);
        self
    }

    fn get_time_till_action(&self) -> u32 {
        // TODO: pick maximum of all players
        self.player_state.get_time_till_action()
    }

    fn tick(&mut self, valves: &[NormalizedValve]) {
        let time_passed = self.get_time_till_action();
        let current_flow = get_current_flow(&self.opened_valves, valves);

        self.accumulated_flow += current_flow * min(time_passed, self.time_left);
        self.time_left = self.time_left.saturating_sub(time_passed);

        // TODO: handle multiplayer.
        self.player_state.tick(time_passed);
    }

    fn apply_player_actions(&mut self) {
        // TODO: handle multiplayer: apply to all players.
        match &self.player_state {
            PlayerState::Wait { .. } => {}
            PlayerState::Travel { .. } => {}
            PlayerState::Open { at: id, time_left } => {
                if *time_left == 0 {
                    self.opened_valves.insert(*id);
                }
            }
        }
    }

    fn get_next_states(self, valves: &[NormalizedValve], distances: &[Vec<u32>]) -> Vec<State> {
        // TODO: handle multiplayer: generate cross-product.
        self.player_state
            .clone()
            .get_next_states(&self, valves, distances)
            .into_iter()
            .map(|player_state| {
                State {
                    player_state,
                    ..self.clone()
                }
                .update_estimated_total_flow(valves)
            })
            .collect::<Vec<_>>()
    }
}

// We compare States by estimated_total_flow, to implement A* with a priority queue.
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

pub fn find_optimal_total_flow(
    starting_at: usize,
    valves: &[NormalizedValve],
    time_left: u32,
) -> u32 {
    let distances = all_pairs_shortest(valves);

    let mut state_priority_queue = BinaryHeap::<State>::new();
    state_priority_queue.push(State {
        player_state: PlayerState::Wait {
            at: starting_at,
            time_left: 0,
        },
        opened_valves: BitSet::new(),
        accumulated_flow: 0,
        time_left,
        estimated_total_flow: u32::MAX,
    });

    let mut best_solution_so_far = u32::MIN;

    while let Some(state) = state_priority_queue.pop() {
        let mut state = state;

        best_solution_so_far = max(best_solution_so_far, state.accumulated_flow);

        state.tick(valves);
        state.apply_player_actions();
        for state in state.get_next_states(valves, &distances) {
            if state.estimated_total_flow > best_solution_so_far {
                state_priority_queue.push(state);
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
