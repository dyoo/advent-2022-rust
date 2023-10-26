use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct State {
    pub at: usize,
    pub open: BitSet,
}

pub fn find_optimal_total_flow(
    starting_at: usize,
    valves: &[NormalizedValve],
    time_left: usize,
) -> u32 {
    let start_state = dynamic_programming::State {
        at: starting_at,
        open: BitSet::new(),
    };
    let cache = &mut HashMap::new();
    get_optimal_total_flow_internal(&start_state, valves, time_left, cache)
}

pub fn get_optimal_total_flow_internal(
    state: &State,
    valves: &[NormalizedValve],
    time_left: usize,
    cache: &mut HashMap<(State, usize), u32>,
) -> u32 {
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
        if !state.open.contains(current_valve.id) && current_valve.flow_rate > 0 {
            let new_state = &State {
                at: state.at,
                open: {
                    let mut new_open = state.open.clone();
                    new_open.insert(current_valve.id);
                    new_open
                },
            };
            get_optimal_total_flow_internal(new_state, valves, time_left - 1, cache)
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
        get_optimal_total_flow_internal(&new_state, valves, time_left - 1, cache)
    });

    let best_result =
        current_flow + std::cmp::max(scores_after_moving.max().unwrap_or(0), score_after_opening);

    cache.insert((state.clone(), time_left), best_result);

    best_result
}
