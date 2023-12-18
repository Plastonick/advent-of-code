use crate::common::get_lines;
use crate::Args;
use std::collections::{HashMap, HashSet};

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
    let (height, width) = (
        lines.len() as isize * 2,
        lines.first().unwrap().len() as isize * 2,
    );
    let inner_points = find_inner_points(&lines, (height, width), s_loop);

    (furthest_from_s.to_string(), inner_points.len().to_string())
}

fn find_inner_points(
    lines: &Vec<String>,
    size: (isize, isize),
    s_loop: Vec<Position>,
) -> HashSet<Position> {
    // to allow us to draw rays to points without awkward interactions, we'll shift all the points
    // slightly out of the plane
    let expanded_map = lines
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, _)| (((r * 2) + 1) as isize, ((c * 2) + 1) as isize))
                .collect::<Vec<Position>>()
        })
        .flatten()
        .collect::<HashSet<Position>>();

    // we'll also need to shift the loop, then fill in any gaps
    let expanded_loop = s_loop
        .iter()
        .map(|p| (p.0 * 2, p.1 * 2))
        .collect::<Vec<_>>();
    let filled_loop = expanded_loop
        .iter()
        .enumerate()
        .map(|(i, &pos)| {
            let next_pos = expanded_loop[(i + 1) % expanded_loop.len()];
            let mid_pos = ((pos.0 + next_pos.0) / 2, (pos.1 + next_pos.1) / 2);

            vec![pos, mid_pos]
        })
        .flatten()
        .collect::<HashSet<_>>();

    // now we just need to draw rays from offset positions from the side of the map and keep a count
    // of the number of intersections of the loop. Odd number => inner point.

    let (height, width) = size;
    let mut inner_points = HashSet::new();

    for r in 0..height {
        let mut intersects = 0;
        for c in 0..width {
            let pos = (r, c);

            if filled_loop.contains(&pos) {
                intersects += 1;
            } else {
                if expanded_map.contains(&pos) && intersects % 2 == 1 {
                    inner_points.insert((r, c));
                }
            }
        }
    }

    // we have counted some "new" positions though that aren't strictly a tile originally! Remap the
    // inner points and ignore any that map to the loop

    inner_points
        .iter()
        .map(|(r, c)| (r - 1, c - 1))
        .filter(|x| !filled_loop.contains(x))
        .collect()
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
