use crate::common::{get_lines, rotate_90, Answer};
use crate::maps::Vector;
use crate::Args;
use pathfinding::prelude::dijkstra_all;
use std::collections::{HashMap, VecDeque};

pub fn run(_args: &Args) -> Answer {
    let (lines, steps_part_1, steps_part_2) = if _args.test {
        (get_lines("day21-test"), 6, 5000) // expect 16733044
    } else {
        (get_lines("day21"), 64, 26501365)
    };

    let (start_point, garden_plots) = parse_garden(&lines);

    let part_1 = positions_after_steps_brute(start_point, &garden_plots, steps_part_1);
    let part_2 = positions_after_steps_geometric(start_point, &garden_plots, steps_part_2);

    (part_1.to_string(), part_2.to_string())
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum GardenPlot {
    Rock,
    Garden,
}

type Garden = Vec<Vec<GardenPlot>>;

fn parse_garden(lines: &Vec<String>) -> (Vector, Garden) {
    let mut start_point = None;
    let garden_plots = lines
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, ch)| {
                    if ch != '#' {
                        let pos = Vector {
                            row: row as isize,
                            col: col as isize,
                        };

                        if ch == 'S' {
                            start_point = Some(pos);
                        }

                        GardenPlot::Garden
                    } else {
                        GardenPlot::Rock
                    }
                })
                .collect()
        })
        .collect();

    (start_point.expect("Did not find start point"), garden_plots)
}

fn positions_after_steps_brute(start: Vector, garden_plots: &Garden, steps: usize) -> usize {
    let all = nodes_within_n(start, steps, |pos| {
        get_neighbours_wrapped(&pos, &garden_plots)
    });

    let parity = steps % 2;
    all.values().filter(|d| *d % 2 == parity).count()
}

fn positions_after_steps_geometric(start: Vector, garden_plots: &Garden, steps: usize) -> usize {
    // we'll do the geometric solution with the input hack...
    let all = dijkstra_all(&start, |v| get_neighbours(&v, &garden_plots));

    let mut visited = all.values().map(|(_, dist)| *dist).collect::<Vec<_>>();
    visited.push(0); // include the starting position

    let even_full = visited.iter().filter(|d| *d % 2 == 0).count();
    let odd_full = visited.iter().filter(|d| *d % 2 == 1).count();

    let odd_corners = visited.iter().filter(|d| *d % 2 == 1 && **d > 65).count();

    let dim = garden_plots.len();
    let n = (steps - (dim / 2)) / dim;

    let saturated = ((n + 1) * (n + 1) * odd_full) + ((n * n) * even_full);
    let odd_subtract = (n + 1) * odd_corners;
    let even_add = sum_partial_evens(n, &garden_plots);

    (saturated + even_add) - odd_subtract
}

fn sum_partial_evens(n: usize, garden_plots: &Garden) -> usize {
    // our partial even-parity squares need to be looked at from the corner, not the middle
    let mut rotated = garden_plots.clone();
    let start = Vector { row: 0, col: 0 };
    let mut sum = 0;
    for _ in 0..4 {
        let mut rot_count = 1; // we can always reach the start plot!
        rotated = rotate_90(rotated);

        rot_count += dijkstra_all(&start, |v| get_neighbours(&v, &rotated))
            .iter()
            .filter(|(_, (_, d))| d % 2 == 0 && *d <= 65)
            .count();

        sum += n * rot_count;
    }

    sum
}

fn get_neighbours(pos: &Vector, garden_plots: &Garden) -> Vec<(Vector, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(row, col)| Vector { row, col })
        .map(|d| pos.add(&d))
        .filter_map(|new_pos| {
            let row_index = new_pos.row as usize;
            let col_index = new_pos.col as usize;

            if row_index >= garden_plots.len() {
                return None;
            }

            if col_index >= garden_plots[0].len() {
                return None;
            }

            if garden_plots[row_index][col_index] == GardenPlot::Garden {
                Some((new_pos, 1))
            } else {
                None
            }
        })
        .collect()
}

fn get_neighbours_wrapped(pos: &Vector, garden_plots: &Garden) -> Vec<Vector> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(row, col)| Vector { row, col })
        .map(|d| d.add(pos))
        .filter_map(|new_pos| {
            let row = new_pos.row as usize % garden_plots.len();
            let col = new_pos.col as usize % garden_plots[0].len();
            if garden_plots[row][col] == GardenPlot::Garden {
                Some(new_pos)
            } else {
                None
            }
        })
        .collect()
}

fn nodes_within_n<N, FN, IN>(start: N, max_steps: usize, successors: FN) -> HashMap<N, usize>
where
    N: Eq + std::hash::Hash + Clone,
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();

    dist.insert(start.clone(), 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let d = dist[&node];

        if d == max_steps {
            continue;
        }

        for next in successors(&node) {
            if !dist.contains_key(&next) {
                dist.insert(next.clone(), d + 1);
                queue.push_back(next);
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_ans() {
        let args = Args {
            day: 21,
            all: true,
            visual: true,
            no_answers: true,
            test: true,
            time: true,
        };

        let ans = run(&args);

        assert_eq!(ans.0, "16");
    }
}
