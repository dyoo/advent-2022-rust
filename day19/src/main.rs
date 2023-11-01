use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash)]
struct Currency {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Currency {
    fn div(&self, other: Currency) -> u32 {
        [
            if other.ore == 0 {
                u32::MAX
            } else {
                self.ore / other.ore
            },
            if other.clay == 0 {
                u32::MAX
            } else {
                self.clay / other.clay
            },
            if other.obsidian == 0 {
                u32::MAX
            } else {
                self.obsidian / other.obsidian
            },
            if other.geode == 0 {
                u32::MAX
            } else {
                self.geode / other.geode
            },
        ]
        .into_iter()
        .min()
        .unwrap_or(0)
    }

    fn sub(&self, other: Currency) -> Currency {
        Currency {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }

    fn scalar_mul(&self, s: u32) -> Currency {
        Currency {
            ore: self.ore * s,
            clay: self.clay * s,
            obsidian: self.obsidian * s,
            geode: self.geode * s,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Blueprint {
    ore: Currency,
    clay: Currency,
    obsidian: Currency,
    geode: Currency,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    purse: Currency,
    time_left: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn new() -> Self {
        State {
            purse: Currency::default(),
            time_left: 24,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

fn get_neighbors(state: &State, blueprint: &Blueprint) -> Vec<State> {
    let mut neighbors: Vec<State> = vec![state.clone()];

    // Greedily buy geode robots.
    neighbors = neighbors
        .into_iter()
        .map(|s| {
            let to_purchase = s.purse.div(blueprint.geode);
            State {
                purse: s.purse.sub(blueprint.geode.scalar_mul(to_purchase)),
                geode_robots: s.geode_robots + to_purchase,
                ..s
            }
        })
        .collect();

    neighbors = neighbors
        .into_iter()
        .flat_map(|s| {
            (0..=(s.purse.div(blueprint.obsidian)))
                .rev()
                .into_iter()
                .map(move |to_purchase| State {
                    purse: s.purse.sub(blueprint.obsidian.scalar_mul(to_purchase)),
                    obsidian_robots: s.obsidian_robots + to_purchase,
                    ..s
                })
        })
        .collect();

    neighbors = neighbors
        .into_iter()
        .flat_map(|s| {
            (0..=(s.purse.div(blueprint.clay)))
                .rev()
                .into_iter()
                .map(move |to_purchase| State {
                    purse: s.purse.sub(blueprint.clay.scalar_mul(to_purchase)),
                    clay_robots: s.clay_robots + to_purchase,
                    ..s
                })
        })
        .collect();

    neighbors = neighbors
        .into_iter()
        .flat_map(|s| {
            (0..=(s.purse.div(blueprint.ore)))
                .into_iter()
                .map(move |to_purchase| State {
                    purse: s.purse.sub(blueprint.ore.scalar_mul(to_purchase)),
                    ore_robots: s.ore_robots + to_purchase,
                    ..s
                })
        })
        .collect();

    // Now harvest, after buying robots.
    for neighbors in neighbors.iter_mut() {
        neighbors.purse.ore += state.ore_robots;
        neighbors.purse.clay += state.clay_robots;
        neighbors.purse.obsidian += state.obsidian_robots;
        neighbors.purse.geode += state.geode_robots;

        neighbors.time_left -= 1;
    }

    neighbors
}

// Compute the quality of a blueprint, optimizing number of geodes.
fn optimize_geodes(blueprint: &Blueprint) -> u32 {
    let state = State::new();
    let mut best = 0;

    fn search(state: &State, blueprint: &Blueprint, best: &mut u32) -> u32 {
        if state.time_left <= 1 {
            let result = state.purse.geode + state.geode_robots * state.time_left;
            if result > *best {
                println!("new best: {} {:?}", result, state);
                *best = result;
            }
            return result;
        }

        let neighbors: Vec<State> = get_neighbors(state, blueprint);
        //        println!("{:?}", neighbors);

        // Search neighbors, pick maximum.
        neighbors
            .into_iter()
            .map(|n| search(&n, blueprint, best))
            .max()
            .unwrap()
    }

    search(&state, blueprint, &mut best)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimize_geodes_small_example() {
        let b = Blueprint {
            ore: Currency {
                ore: 4,
                ..Currency::default()
            },
            clay: Currency {
                ore: 2,
                ..Currency::default()
            },
            obsidian: Currency {
                ore: 3,
                clay: 14,
                ..Currency::default()
            },
            geode: Currency {
                ore: 2,
                obsidian: 7,
                ..Currency::default()
            },
        };
        assert_eq!(optimize_geodes(&b), 9);
    }
}
