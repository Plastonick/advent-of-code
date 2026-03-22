use crate::common::{get_lines, Answer};
use crate::Args;
use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i32, i32);

pub fn run(_args: &Args) -> Answer {
    let map = if _args.test {
        get_lines("day23-test")
    } else {
        get_lines("day23")
    }
    .into_iter()
    .map(|l| l.chars().collect())
    .collect::<Vec<Vec<_>>>();

    let (slippy_graph, slippy_node_map) = decompile(&map, true);

    let (graph, node_map) = decompile(&map, false);

    let target = (map.len() as i32 - 1, map.len() as i32 - 2);
    let slippy_target_index = *slippy_node_map.get(&target).expect("No slippy target");
    let target_index = *node_map.get(&target).expect("No non-slippy target");

    let part1 = longest_path(&slippy_graph, slippy_target_index);
    let part2 = longest_path(&graph, target_index);

    (part1.to_string(), part2.to_string())
}

fn longest_path(graph: &Vec<Vec<(usize, u32)>>, target_index: usize) -> u32 {
    let mut paths: VecDeque<(HashSet<usize>, usize, u32)> = VecDeque::new();
    paths.push_back((HashSet::from([0]), 0, 0));

    let mut max_walk = 0;

    while let Some((visited, at, dist)) = paths.pop_front() {
        let adjacencies = graph[at]
            .iter()
            .filter(|(adj_index, _)| !visited.contains(adj_index));

        for (adj_index, extra_dist) in adjacencies {
            let new_dist = dist + extra_dist;

            if adj_index == &target_index {
                max_walk = max_walk.max(new_dist);
            } else {
                let mut new_visited = visited.clone();
                new_visited.insert(*adj_index);
                paths.push_back((new_visited, *adj_index, new_dist));
            }
        }
    }

    max_walk
}
// 0         5        10        15        20   22
// . A . . . . . . . . . . . . . . . . . . . . .  0
// . # # # # # # # . . . . . . . . . # # # . . .  1
// . . . . . . . # . . . . . . . . . # . # . . .  2
// . . . # # # # # . # > C > # . . . # . # . . .  3
// . . . v . . . . . # . v . # . . . # . # . . .  4
// . . . B > # # # . # . # . # # # # # . # # # .  5
// . . . v . . . # . # . # . . . . . . . . . # .  6
// . . . # # # . # . # . # # # # # # # . # # # .  7
// . . . . . # . # . # . . . . . . . # . # . . .  8
// . # # # # # . # . # . # # # # # # # . # # # .  9
// . # . . . . . # . # . # . . . . . . . . . v . 10
// . # . # # # . # # # . # # # . . . # # # > H . 11
// . # . # . v . . . . . . . v . . . # . . . v . 12
// . # # # . D > # . # # # > G > # . # . . . # . 13
// . . . . . v . # . # . . . v . # . # . . . # . 14
// . # # # # # . # # # . # # # . # . # . # # # . 15
// . # . . . . . . . . . # . . . # . # . # . . . 16
// . # # # . . . # # # . # # # . # # # . # . . . 17
// . . . # . . . # . # . . . v . . . . . v . . . 18
// . # # # . # # # . # . # > E > # . # > F . . . 19
// . # . . . # . . . # . # . . . # . # . v . . . 20
// . # # # # # . . . # # # . . . # # # . # # # . 21
// . . . . . . . . . . . . . . . . . . . . . X . 22

// A=(0, 1),  B=(5, 3),  C=(3, 11),  D=(13, 5),  E=(19, 13),  F=(19, 19),  G=(13, 13),  H=(11, 21),  X=(22, 21)

// A, B, C, D, G, X ... no E, F, or H

fn decompile(
    map: &Vec<Vec<char>>,
    slippy: bool,
) -> (Vec<Vec<(usize, u32)>>, HashMap<Point, usize>) {
    // identify all forks in the road, each fork becomes a node
    // each node contains information of the nodes it links to and the length of each link
    let mut node_map: HashMap<Point, usize> = HashMap::from_iter([((0, 1), 0)]);
    let mut graph: Vec<Vec<(usize, u32)>> = vec![vec![]];

    // prev node, prev point, curr_point, dist from prev node
    let mut paths: VecDeque<(usize, Point, Point, u32)> = VecDeque::new();
    paths.push_back((0, (0, 1), (1, 1), 1));

    while let Some((prev_node_index, prev_point, curr_point, dist_from_prev_node)) =
        paths.pop_front()
    {
        // get possible adjacencies
        let valid_adjacencies = [(1, 0, 'v'), (0, 1, '>'), (-1, 0, '^'), (0, -1, '<')]
            .into_iter()
            .filter_map(|d| {
                let p = (curr_point.0 + d.0, curr_point.1 + d.1);

                if p.0 < 0 || p.1 < 0 {
                    return None;
                }

                if p.0 >= map.len() as i32 || p.1 >= map[p.0 as usize].len() as i32 {
                    return None;
                }

                match map[p.0 as usize][p.1 as usize] {
                    '.' => Some((p, true)),
                    '#' => None,
                    c => Some((p, !slippy || c == d.2)),
                }
            })
            .filter(|(p, _)| p != &prev_point)
            .collect::<Vec<_>>();

        // a point is a node if there's not a single valid way to go (either a fork or an end)
        let (prev_node_index, dist_from_prev_node) = if valid_adjacencies.len() != 1 {
            let curr_node_index = *node_map.entry(curr_point).or_insert_with(|| {
                graph.push(Vec::new());
                graph.len() - 1
            });

            node_map.insert(curr_point, curr_node_index);

            if graph[prev_node_index].contains(&(curr_node_index, dist_from_prev_node)) {
                continue;
            }

            // link the two nodes to each other
            graph[prev_node_index].push((curr_node_index, dist_from_prev_node));
            graph[curr_node_index].push((prev_node_index, dist_from_prev_node));

            (curr_node_index, 0)
        } else {
            (prev_node_index, dist_from_prev_node)
        };

        for (adj, can_progress) in valid_adjacencies {
            if !can_progress {
                continue;
            }

            paths.push_back((prev_node_index, curr_point, adj, dist_from_prev_node + 1));
        }
    }

    (graph, node_map)
}
