use crate::common::{get_lines, Answer};
use crate::maps::{Vector, _print_vec};
use crate::Args;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Particle {
    position: Vector,
    direction: Vector,
    count: i8,
    value: usize,
    path: Vec<Vector>,
}

struct Map {
    tiles: HashMap<Vector, usize>,
    size: Vector,
}

static MAX_LINE: u8 = 3;

impl Vector {
    fn successors(
        &self,
        map: &Map,
        direction: &Vector,
        count: &u8,
    ) -> Vec<((Vector, Vector, u8), usize)> {
        fn in_bounds(x: &Vector, map: &Map) -> bool {
            x.row >= 0 && x.col >= 0 && x.row <= map.size.row && x.col <= map.size.col
        }

        let directions = vec![
            Vector { row: 1, col: 0 },
            Vector { row: -1, col: 0 },
            Vector { row: 0, col: 1 },
            Vector { row: 0, col: -1 },
        ];

        directions
            .into_iter()
            .filter(|d| !d.eq(&direction.mul(-1)))
            .filter(|d| !d.eq(&direction) || count < &MAX_LINE)
            .map(|d| {
                let new_count = if d.eq(&direction) { count + 1 } else { 1 };
                (self.add(&d), d, new_count)
            })
            .filter(|(x, _, _)| in_bounds(&x, &map))
            .map(|x| (x, *map.tiles.get(&x.0).unwrap()))
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

    let result = dijkstra(
        &(Vector { row: 0, col: 0 }, Vector { row: 0, col: 0 }, 0_u8),
        |(x, direction, count)| x.successors(&map, &direction, count),
        |(x, _, _)| x.eq(&Vector { row, col }),
    )
    .expect("No path found");

    dbg!(&result);

    _print_vec(
        &result
            .0
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<Vector>>(),
    );

    (result.1.to_string(), "".to_string())
}
