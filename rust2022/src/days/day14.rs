use std::{cmp::max, collections::HashSet};

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day14-test")
    } else {
        get_lines("day14")
    };

    let (mut blocks, max_depth) = build_solid_surfaces(&lines);

    let volume = count_area_under((500, 0), &blocks, max_depth + 2);

    let mut escaped = false;
    let mut sand_pieces = 0;

    while !escaped {
        // place a piece of sand at (0, 500) and watch it fall
        let mut sand = (500, 0);
        sand_pieces += 1;

        loop {
            let sand_result = move_sand(sand, &blocks, max_depth + 2);

            // the sand is stuck, add it to the blocks then drop another peice of sand
            if sand_result.is_none() {
                blocks.insert(sand);

                break;
            } else {
                sand = sand_result.unwrap();

                if sand.1 >= max_depth && !escaped {
                    // we've escaped!
                    escaped = true;
                }
            }
        }

        // our sand can't move, break
        if sand == (500, 0) {
            break;
        }
    }

    if !args.no_answers {
        println!(
            "Day 14, Part 1: It took {} pieces of sand to start overflowing",
            sand_pieces - 1
        );
        println!(
            "Day 14, Part 2: It took {} pieces of sand to make a big pyramid",
            volume
        );
    }

    ("".to_string(), "".to_string())
}

fn move_sand(
    sand: (isize, isize),
    blocks: &HashSet<(isize, isize)>,
    floor_depth: isize,
) -> Option<(isize, isize)> {
    // are we at the infinite floor? If so, we can't move, return None
    if sand.1 == floor_depth - 1 {
        return None;
    }

    // can I go straight down?
    let straight_down = (sand.0, sand.1 + 1);

    if !blocks.contains(&straight_down) {
        return Some(straight_down);
    }

    // okay, can I go down-and-left?
    let down_and_left = (sand.0 - 1, sand.1 + 1);

    if !blocks.contains(&down_and_left) {
        return Some(down_and_left);
    }

    // okay, what about down and right?
    let down_and_right = (sand.0 + 1, sand.1 + 1);

    if !blocks.contains(&down_and_right) {
        return Some(down_and_right);
    }

    // I can't move, return none
    None
}

fn count_area_under(
    point: (isize, isize),
    blocks: &HashSet<(isize, isize)>,
    floor: isize,
) -> isize {
    // path map from the source to the sink, remembering all visited nodes
    let mut visited: HashSet<(isize, isize)> = HashSet::from([point]);
    let mut last_sources: HashSet<(isize, isize)> = HashSet::from([point]);

    while last_sources.len() > 0 {
        let mut sources: HashSet<(isize, isize)> = HashSet::new();

        for source in &last_sources {
            let next_depth = source.1 + 1;

            // it's too low! Leave it out
            if next_depth >= floor {
                // this can probably be break; since each wave should be the same depth...
                continue;
            }

            for d in -1..=1 {
                let next = (source.0 + d, next_depth);

                // it's blocked! Leave it out
                if blocks.contains(&next) {
                    continue;
                }

                // it's good! Include it
                sources.insert(next);
                visited.insert(next);
            }
        }

        last_sources = sources;
    }

    visited.len() as isize
}

fn build_solid_surfaces(lines: &Vec<String>) -> (HashSet<(isize, isize)>, isize) {
    let mut points = HashSet::new();

    for line in lines {
        let coords: Vec<_> = line
            .split(" -> ")
            .map(|a| {
                let (x, y) = a.split_once(',').unwrap();
                (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
            })
            .collect();

        for (i, coord) in coords[1..].iter().enumerate() {
            let mut prev_coord = coords[i];
            let delta = (coord.0 - prev_coord.0, coord.1 - prev_coord.1);
            let unit_direction = (
                delta.0 / max(delta.0.abs(), 1),
                delta.1 / max(delta.1.abs(), 1),
            );

            while prev_coord != *coord {
                points.insert(prev_coord);

                prev_coord = (
                    prev_coord.0 + unit_direction.0,
                    prev_coord.1 + unit_direction.1,
                );
            }

            // insert the end point of the line, too!
            points.insert(prev_coord);
        }
    }

    let (_, max_depth) = points
        .clone()
        .into_iter()
        .reduce(|a, b| (0, max(a.1, b.1)))
        .unwrap();

    (points, max_depth)
}
