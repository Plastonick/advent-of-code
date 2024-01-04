use crate::common::get_lines;
use crate::maps::Vector;
use crate::Args;
use std::collections::HashMap;

type Position = (isize, isize);
type Map = HashMap<Position, (Position, Position)>;

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day10-test")
    } else {
        get_lines("day10")
    };

    let map: Map = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(column, tile)| map_tile((row as isize, column as isize), tile))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Map>();

    let s_pos = *lines
        .iter()
        .enumerate()
        .filter_map(|(row, l)| {
            if let Some(col) = l.find('S') {
                Some((row as isize, col as isize))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    let mut s_loop = vec![];
    let mut prev = s_pos;

    if let Some(mut current) = s_neighbour(&s_pos, &map) {
        s_loop.push(current);

        while !eq(&current, &s_pos) {
            (prev, current) = move_loop(prev, current, &map);

            s_loop.push(current);
        }
    }

    let furthest_from_s = s_loop.len() / 2;

    // part 2 we're going to be clever and count the number of loop intersections by tracing a ray
    // from the edge of the map and counting the number of loop intersections.
    // This is _slightly_
    let s_loop_as_vecs = s_loop
        .into_iter()
        .map(|(row, col)| Vector { row, col })
        .collect::<Vec<_>>();
    let inner_points = crate::maps::find_inner_points(&s_loop_as_vecs);

    (furthest_from_s.to_string(), inner_points.len().to_string())
}

fn map_tile(pos: Position, tile: char) -> Option<(Position, (Position, Position))> {
    let has_neighbours = match tile {
        '|' => Some(((-1, 0), (1, 0))),
        '-' => Some(((0, -1), (0, 1))),
        'L' => Some(((-1, 0), (0, 1))),
        'J' => Some(((0, -1), (-1, 0))),
        '7' => Some(((0, -1), (1, 0))),
        'F' => Some(((1, 0), (0, 1))),
        'S' => None, // TODO ?
        _ => None,
    };

    if let Some(neighbours) = has_neighbours {
        Some((pos, (add(&pos, &neighbours.0), add(&pos, &neighbours.1))))
    } else {
        None
    }
}

fn s_neighbour(s_pos: &Position, map: &Map) -> Option<Position> {
    for i in -1..=1 {
        for j in -1..=1 {
            // we don't want diagonal neighbours
            if i * j != 0 {
                continue;
            }

            let neighbor_pos = add(&s_pos, &(i, j));
            if let Some(neighbour) = map.get(&neighbor_pos) {
                if eq(&neighbour.0, &s_pos) || eq(&neighbour.1, &s_pos) {
                    return Some(neighbor_pos);
                }
            }
        }
    }

    None
}

fn add(a: &Position, b: &Position) -> Position {
    (a.0 + b.0, a.1 + b.1)
}

fn eq(a: &Position, b: &Position) -> bool {
    a.0 == b.0 && a.1 == b.1
}

// each entry should connect to exactly two others, so find the one not connecting to the previous one!
fn move_loop(prev: Position, current: Position, map: &Map) -> (Position, Position) {
    let (a, b) = map.get(&current).unwrap();

    if eq(&a, &prev) {
        (current, *b)
    } else {
        (current, *a)
    }
}
