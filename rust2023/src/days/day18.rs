use crate::common::{get_lines, Answer};
use crate::maps::{_print, find_inner_points, Vector};
use crate::Args;
use std::collections::HashSet;

pub fn run(_args: &Args) -> Answer {
    let lines = if _args.test {
        get_lines("day18-test")
    } else {
        get_lines("day18")
    };

    let instructions = lines
        .iter()
        .map(|line| {
            let (dir, rest) = line.split_once(' ').unwrap();
            let (count, colour) = rest.split_once(' ').unwrap();

            let direction = match dir {
                "R" => Vector { row: 0, col: 1 },
                "L" => Vector { row: 0, col: -1 },
                "U" => Vector { row: -1, col: 0 },
                "D" => Vector { row: 1, col: 0 },
                _ => {
                    panic!("Unexpected direction '{dir}'")
                }
            };

            (direction, count.parse::<isize>().unwrap(), colour)
        })
        .collect::<Vec<_>>();

    let mut digger = Vector { row: 0, col: 0 };
    let mut path = vec![digger];
    for (direction, count, _) in instructions {
        for _ in 0..count {
            digger = digger.add(&direction);

            path.push(digger);
        }
    }

    let path_points = path.clone().into_iter().collect::<HashSet<Vector>>();
    let mut points = find_inner_points(&path);
    points.extend(path_points);

    _print(&points);

    (points.len().to_string(), "".to_string())
}
