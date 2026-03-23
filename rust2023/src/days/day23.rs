use crate::common::{get_lines, Answer};
use crate::Args;
use std::collections::{HashMap, VecDeque};

type Point = (i32, i32);

struct Visited(u64);

impl Visited {
    fn not_visited(&self, other: usize) -> bool {
        let mask = 2u64.pow(other as u32);

        mask & self.0 == 0
    }

    fn with(&self, other: usize) -> Visited {
        let mask = 2u64.pow(other as u32);

        Visited(self.0 + mask)
    }
}

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

    // I tried a DP-DFS, but this was a few times slower than an exhaustive BFS with bit-masked state

    let part1 = longest_path(&slippy_graph, slippy_target_index);
    let part2 = longest_path(&graph, target_index);

    (part1.to_string(), part2.to_string())
}

fn longest_path(graph: &Vec<Vec<(usize, u32)>>, target_index: usize) -> u32 {
    let mut paths: VecDeque<(Visited, usize, u32)> = VecDeque::new();
    paths.push_back((Visited(1), 0, 0));
    let mut max_walk = 0;

    // I did briefly add a little memoization in here; check if we've already got to the same (at, visited) state, and
    // if we have but at a greater distance; then ignore that path, but it turned out to be slower. Possibly due to the
    // hash lookups which we've managed to avoid here, now.

    while let Some((visited, at, dist)) = paths.pop_front() {
        let adjacencies = graph[at]
            .iter()
            .filter(|(adj_index, _)| visited.not_visited(*adj_index));

        for (adj_index, extra_dist) in adjacencies {
            let new_dist = dist + extra_dist;

            if adj_index == &target_index {
                max_walk = max_walk.max(new_dist);
            } else {
                paths.push_back((visited.with(*adj_index), *adj_index, new_dist));
            }
        }
    }

    max_walk
}

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
                    '.' => Some(p),
                    '#' => None,
                    c => {
                        if !slippy || c == d.2 {
                            Some(p)
                        } else {
                            None
                        }
                    }
                }
            })
            .filter(|p| p != &prev_point)
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

            if !slippy {
                // only link them both ways if the ^v<> are reversible
                graph[curr_node_index].push((prev_node_index, dist_from_prev_node));
            }

            (curr_node_index, 0)
        } else {
            (prev_node_index, dist_from_prev_node)
        };

        for adj in valid_adjacencies {
            paths.push_back((prev_node_index, curr_point, adj, dist_from_prev_node + 1));
        }
    }

    (graph, node_map)
}
