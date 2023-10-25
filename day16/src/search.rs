use crate::*;

fn all_pairs_shortest(valves: &[NormalizedValve]) -> Vec<Vec<usize>> {
    let n = valves.len();

    let mut costs = vec![vec![usize::MAX; n]; n];
    // Initial distances
    for (i, valve) in valves.iter().enumerate() {
        for exit in &valve.exits {
            costs[i][*exit] = 1;
        }
    }

    floyd_warshall(costs)
}

fn floyd_warshall(mut costs: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
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

#[derive(Debug)]
pub struct State {}

pub fn find_optimal_total_flow(
    start_state: &State,
    valves: &[NormalizedValve],
    time_left: usize,
) -> i32 {
    0
}

#[test]
fn test_floyd_warshall() {
    let inf = usize::MAX;

    //
    // x <----> y <-----> z
    //
    let input = vec![vec![0, 1, inf], vec![1, 0, 1], vec![inf, 1, 0]];
    let output = floyd_warshall(input);

    assert_eq!(output, vec![vec![0, 1, 2], vec![1, 0, 1], vec![2, 1, 0],]);
}
