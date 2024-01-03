use crate::common::{get_lines, Answer};
use crate::maps::{Vector, _print_vec};
use crate::Args;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

struct Map {
    tiles: HashMap<Vector, usize>,
    size: Vector,
}

impl Vector {
    fn successors(
        &self,
        map: &Map,
        previous_direction: &Vector,
        (min, max): &(isize, isize), // how many times we must move in the next direction
    ) -> Vec<((Vector, Vector), usize)> {
        fn in_bounds(x: &Vector, map: &Map) -> bool {
            x.row >= 0 && x.col >= 0 && x.row <= map.size.row && x.col <= map.size.col
        }

        let directions = if previous_direction.row == previous_direction.col {
            vec![Vector { row: 1, col: 0 }, Vector { row: 0, col: 1 }]
        } else {
            vec![
                Vector {
                    row: previous_direction.col,
                    col: previous_direction.row,
                },
                Vector {
                    row: -previous_direction.col,
                    col: -previous_direction.row,
                },
            ]
        };

        // possible delta paths to take from current position
        // every combination of directions and numbers in (min..=max) range
        let deltas = directions
            .iter()
            .map(|d| {
                (*min..=*max)
                    .into_iter()
                    .map(|steps| {
                        (1..=steps)
                            .into_iter()
                            .map(|step| (d.mul(step), d.clone()))
                            .collect::<Vec<(Vector, Vector)>>()
                    })
                    .collect::<Vec<Vec<(Vector, Vector)>>>()
            })
            .flatten()
            .collect::<Vec<Vec<(Vector, Vector)>>>();

        deltas
            .into_iter()
            .map(|deltas| {
                deltas
                    .into_iter()
                    .map(|(delta, dir)| (self.add(&delta), dir))
                    .collect::<Vec<(Vector, Vector)>>()
            })
            .filter(|path| in_bounds(&path.last().unwrap().0, &map))
            .map(|path| {
                // dbg!(&path);

                let cost = path
                    .iter()
                    .map(|(pos, _)| map.tiles.get(&pos).unwrap())
                    .sum::<usize>();

                // we care about where we end up, and the cost it takes to get there
                (path.last().unwrap().clone(), cost)
            })
            .collect()
    }
}

pub fn run(args: &Args) -> Answer {
    let lines = if args.test {
        get_lines("day17-test")
    } else {
        get_lines("day17")
    };

    let tiles = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| {
                    let val: usize = ch.to_string().parse().unwrap();

                    (
                        Vector {
                            row: row as isize,
                            col: col as isize,
                        },
                        val,
                    )
                })
                .collect::<Vec<(Vector, usize)>>()
        })
        .flatten()
        .collect::<HashMap<Vector, usize>>();

    let (row, col) = tiles
        .iter()
        .map(|(a, _)| (a.row, a.col))
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();
    let map = Map {
        tiles,
        size: Vector { row, col },
    };
    let target = Vector { row, col };

    let result_part_1 = find_path(&map, &target, (1, 3));
    let result_part_2 = find_path(&map, &target, (4, 10));

    (result_part_1.1.to_string(), result_part_2.1.to_string())
}

fn find_path(
    map: &Map,
    target: &Vector,
    stepping: (isize, isize),
) -> (Vec<(Vector, Vector)>, usize) {
    dijkstra(
        &(Vector { row: 0, col: 0 }, Vector { row: 0, col: 0 }),
        |(pos, d)| pos.successors(&map, &d, &stepping),
        |(x, _)| x.eq(&target),
    )
    .expect("No path found")
}
