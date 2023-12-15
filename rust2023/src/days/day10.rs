use crate::common::get_lines;
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

    // part 2 we're going to be clever and count the number of loop intersections
    let (height, width) = (lines.len() as isize, lines.first().unwrap().len() as isize);

    let mut inner_points = 0;
    for row in -1..=height {
        for col in -1..=width {
            let intersects = intersect_points_to(&(row, col), &s_loop);
            inner_points += intersects % 2;
        }
    }

    (furthest_from_s.to_string(), "TODO".to_string())
}

fn intersect_points_to(pos: &Position, s_loop: &Vec<Position>) -> usize {
    let mut intersects = 0;

    // get all loop points between the edge and target position

    let loop_ray = s_loop
        .iter()
        .filter(|&&x| x.0 == pos.0 && x.1 < pos.1)
        .collect::<Vec<_>>();

    5
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
