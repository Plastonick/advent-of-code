use std::collections::HashSet;

use crate::{common::get_lines, Args};

type Point = (i32, i32);
type Direction = (i32, i32);

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day24-test")
    } else {
        get_lines("day24")
    };

    let mut wave = HashSet::from_iter(vec![(-1, 0)]);
    let mut target = (0, 0);
    let mut bounds = HashSet::new();

    let mut storms = Vec::new();

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().into_iter().enumerate() {
            let point = (r as i32 - 1, c as i32 - 1);

            match ch {
                '#' => {
                    bounds.insert(point);
                }
                '>' => storms.push((point, (0, 1))),
                'v' => storms.push((point, (1, 0))),
                '<' => storms.push((point, (0, -1))),
                '^' => storms.push((point, (-1, 0))),
                _ => target = point,
            }
        }
    }

    // don't allow going around the outside!
    bounds.insert((-2, 0));
    bounds.insert((target.0 + 1, target.1));

    // let storm_pos = storms.iter().map(|(p, _)| p).collect::<Vec<_>>();

    // dbg!(target.0, target.1 + 1);
    // panic!();

    let directions = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut tick = 1;
    'main: loop {
        let mut next_wave = HashSet::new();

        // where will the storms be next tick?
        storms = move_storms(storms, (target.0, target.1 + 1));

        // where will a storm be?
        let mut storm_set = HashSet::new();
        for (position, _) in &storms {
            storm_set.insert(position);
        }

        dbg!(storm_set.len());

        // find possible next positions
        for point in wave {
            for direction in directions {
                let new_point = (point.0 + direction.0, point.1 + direction.1);

                if new_point.0 < -1 || new_point.1 < -1 {
                    continue;
                }

                if bounds.contains(&new_point) {
                    continue;
                }

                if storm_set.contains(&new_point) {
                    continue;
                }

                next_wave.insert(new_point);

                if new_point == target {
                    println!("Found the target!");

                    break 'main;
                }
            }
        }

        tick += 1;
        wave = next_wave;
    }

    if !args.no_answers {
        println!("Day 24, Part 1: Takes {} ticks to leave", tick);
    }

    (tick.to_string(), "".to_string())
}

fn move_storms(storms: Vec<(Point, Direction)>, bounds: Point) -> Vec<(Point, Direction)> {
    let mut new_storms = Vec::new();

    for (position, direction) in storms {
        let new_position = (
            (position.0 + direction.0).rem_euclid(bounds.0),
            (position.1 + direction.1).rem_euclid(bounds.1),
        );

        new_storms.push((new_position, direction));
    }

    new_storms
}
