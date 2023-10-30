#[derive(Debug, PartialEq, Eq, Default)]
struct Currency {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    ore_robot_cost: Currency,
    clay_robot_cost: Currency,
    obsidian_robot_cost: Currency,
    geode_robot_cost: Currency,
}

#[derive(Debug)]
struct State {
    holding: Currency,
    time_left: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn new() -> Self {
        State {
            holding: Currency::default(),
            time_left: 24,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

// Compute the quality of a blueprint, optimizing number of geodes.
fn optimize_geodes(blueprint: &Blueprint) -> u32 {
    let state = State::new();

    fn search(blueprint: &Blueprint, state: &State) -> u32 {
        if state.time_left == 0 {
            return state.holding.geode;
        } else if state.time_left == 1 {
            return state.holding.geode + state.geode_robots;
        }

        // Collect neighbors of the state.
        // At any point, perhaps buy 0 or more robots that we can afford.
        //
        // I believe we can be greedy if we can afford a geode robot, but otherwise,
        // we have to balance buying other robots and saving, as ore
        // is used for the construction of all kinds of robots and so
        // compete.
        //
        // Search neighbors, pick maximum.
        0
    }

    search(blueprint, &state)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {}
