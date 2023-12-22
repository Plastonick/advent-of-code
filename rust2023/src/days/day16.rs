use crate::common::{get_lines, Answer, Vector};
use crate::Args;
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

pub fn run(args: &Args) -> Answer {
    let lines = if args.test {
        get_lines("day16-test")
    } else {
        get_lines("day16")
    };

    let map = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| {
                    (
                        Vector {
                            row: row as isize,
                            col: col as isize,
                        },
                        ch,
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<Vector, char>>();

    let top_left = (Vector { row: 0, col: -1 }, Vector { row: 0, col: 1 });
    let visited_from_top_left = energise_map(&map, top_left, args.visual);

    let (height, width) = (lines.len() as isize, lines[0].len() as isize);

    let best_row = (0..height)
        .into_iter()
        .map(|i| {
            let left_entry = (Vector { row: i, col: -1 }, Vector { row: 0, col: 1 });
            let right_entry = (Vector { row: i, col: width }, Vector { row: 0, col: -1 });

            energise_map(&map, left_entry, args.visual)
                .len()
                .max(energise_map(&map, right_entry, args.visual).len())
        })
        .reduce(|a, b| a.max(b))
        .unwrap();

    let best_col = (0..width)
        .into_iter()
        .map(|i| {
            let top_entry = (Vector { row: -1, col: i }, Vector { row: 1, col: 0 });
            let bottom_entry = (
                Vector {
                    row: height,
                    col: i,
                },
                Vector { row: -1, col: 0 },
            );

            let best_top = energise_map(&map, top_entry, args.visual).len();
            let best_bottom = energise_map(&map, bottom_entry, args.visual).len();

            best_top.max(best_bottom)
        })
        .reduce(|a, b| a.max(b))
        .unwrap();

    (
        visited_from_top_left.len().to_string(),
        best_row.max(best_col).to_string(),
    )
}

// TODO performance optimisation
// dynamically cache the set of expected visited nodes for a given ray (position, velocity)
fn energise_map(
    map: &HashMap<Vector, char>,
    entry: (Vector, Vector),
    visualise: bool,
) -> HashSet<Vector> {
    let mut rays = vec![entry];
    let mut visited: HashSet<Vector> = HashSet::new();
    let mut ray_vectors: HashSet<(Vector, Vector)> = HashSet::new();

    while rays.len() > 0 {
        rays = iterate(rays, &map);

        // remove any rays we've already seen, matching both position and velocity
        rays = rays
            .into_iter()
            .filter(|&ray| !ray_vectors.contains(&ray))
            .collect::<Vec<_>>();

        visited.extend(rays.iter().map(|(pos, _)| pos));
        ray_vectors.extend(rays.iter());

        if visualise {
            _print(&map, &visited);
        }
    }

    visited
}

fn _print(map: &HashMap<Vector, char>, visited: &HashSet<Vector>) {
    let size = map
        .iter()
        .map(|(a, _)| (a.row, a.col))
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();

    for row in 0..size.0 {
        for col in 0..size.1 {
            let pos = Vector { row, col };

            let tile = if visited.contains(&pos) {
                '#'
            } else if let Some(tile) = map.get(&pos) {
                *tile
            } else {
                '.'
            };

            print!("{}", tile);
        }

        println!();
    }

    println!();
    println!();
    sleep(Duration::new(0, 50_000_000));
}

fn iterate(rays: Vec<(Vector, Vector)>, map: &HashMap<Vector, char>) -> Vec<(Vector, Vector)> {
    rays.into_iter()
        .map(|(pos, velocity)| {
            let new_pos = pos.add(&velocity);
            let tile = if let Some(tile) = map.get(&new_pos) {
                tile
            } else {
                // we've probably gone off the edge of the map, let it continue
                return vec![];
            };

            // is the light ray just continuing through this tile?
            let is_horz = velocity.row == 0;
            let filters_through = (is_horz && tile == &'-') || (!is_horz && tile == &'|');
            if tile == &'.' || filters_through {
                return vec![(new_pos, velocity)];
            }

            // is the light ray split into two rays?
            let is_split = (is_horz && tile == &'|') || (!is_horz && tile == &'-');
            if is_split {
                let v_a = Vector {
                    row: velocity.col,
                    col: velocity.row,
                };
                let v_b = Vector {
                    row: -velocity.col,
                    col: -velocity.row,
                };

                return vec![(new_pos, v_a), (new_pos, v_b)];
            }

            // otherwise, the light ray must be bouncing!
            let v_c = if tile == &'/' {
                Vector {
                    row: -velocity.col,
                    col: -velocity.row,
                }
            } else if tile == &'\\' {
                Vector {
                    row: velocity.col,
                    col: velocity.row,
                }
            } else {
                panic!("Unexpected tile!");
            };

            vec![(new_pos, v_c)]
        })
        .flatten()
        .collect()
}
