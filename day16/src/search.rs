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
    Travel { id: usize, time_left: u32 },
    Open { id: usize, time_left: u32 },
}

const TIME_TO_OPEN_VALVE: u32 = 1;

// Intended use of PlayerState:
//
// * Use tick to find minimum time of stability (for flow accumulation)
// * Use apply to apply changes the player makes to the state.
// * Use nexts to find new actions for each player.
impl PlayerState {
    fn time_left(&self) -> u32 {
        match self {
            PlayerState::At { .. } => 0,
            PlayerState::Travel { time_left, .. } => *time_left,
            PlayerState::Open { time_left, .. } => *time_left,
        }
    }

    fn destination(&self) -> usize {
        match self {
            PlayerState::At { id } => *id,
            PlayerState::Travel { id, .. } => *id,
            PlayerState::Open { id, .. } => *id,
        }
    }

    fn tick(self, time_passed: u32) -> Self {
        match self {
            PlayerState::At { .. } => self,
            PlayerState::Travel { id, time_left } => PlayerState::Travel {
                id,
                time_left: time_left.saturating_sub(time_passed),
            },
            PlayerState::Open { id, time_left } => PlayerState::Open {
                id,
                time_left: time_left.saturating_sub(time_passed),
            },
        }
    }

    fn apply(self, state: &State) {
        match self {
            PlayerState::At { .. } => {}
            PlayerState::Travel { .. } => {}
            PlayerState::Open { id, time_left } => {
                if time_left == 0 {
                    state.opened_valves.insert(id);
                }
            }
        }
    }

    // Returns list of new player states.
    fn nexts(
        self,
        state: &State,
        valves: &[NormalizedValve],
        distances: &[Vec<u32>],
    ) -> Vec<PlayerState> {
        match self {
            PlayerState::At { id } => {
                // Schedule a visit to a closed valve that has flow.
                let distance_to = &distances[id];

                // TODO: switch when player_state becomes player_states.
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

                accessible_closed_valves
                    .into_iter()
                    .map(|valve| PlayerState::Travel {
                        id: valve.id,
                        time_left: distance_to[valve.id],
                    })
                    .collect()
            }
            PlayerState::Travel { id, time_left } => {
                if time_left == 0 {
                    vec![PlayerState::Open {
                        id,
                        time_left: TIME_TO_OPEN_VALVE,
                    }]
                } else {
                    vec![self]
                }
            }
            PlayerState::Open { id, time_left } => {
                if time_left == 0 {
                    vec![PlayerState::At { id }]
                } else {
                    vec![self]
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

impl State {
    fn update_estimated_total_flow(mut self, valves: &[NormalizedValve]) -> Self {
        self.estimated_total_flow = self.accumulated_flow
            + estimated_flow_heuristic(&self.opened_valves, self.time_left, valves);
        self
    }

    // Simulate passage of time, returning list of new states afterwards.
    fn tick(self, valves: &[NormalizedValve]) -> Vec<Self> {
        // Find how much time has to pass before something interesting
        // happens.
        // let time_passing = self.player_state.time_left();

        // let current_flow = get_current_flow(&self.opened, valves);

        // Generate new candidate states.

        vec![self]
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
                return total_flow + current_flow * (time_left - i - 1);
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
        accumulated_flow: 0,
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
            state.accumulated_flow + state.time_left * current_flow,
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
            let new_total_flow = state.accumulated_flow + new_time_passed * current_flow;
            let new_time_left = state.time_left - new_time_passed;

            let new_state = State {
                player_state: new_player_state,
                opened_valves: new_opened_valves,
                accumulated_flow: new_total_flow,
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
