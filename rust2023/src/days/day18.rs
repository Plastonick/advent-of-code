use crate::common::{get_lines, Answer};
use crate::maps::Vector;
use crate::Args;
use std::isize;

pub fn run(_args: &Args) -> Answer {
    let lines = if _args.test {
        get_lines("day18-test")
    } else {
        get_lines("day18")
    };

    let instructions_part_1 = build_instructions_part1(&lines);
    let instructions_part_2 = build_instructions_part2(&lines);

    (
        find_interior_volume(&instructions_part_1).to_string(),
        find_interior_volume(&instructions_part_2).to_string(),
    )
}

fn find_interior_volume(instructions: &Vec<(Vector, isize)>) -> isize {
    // Finding the area of a polygon with known ordered vertices: https://www.topcoder.com/thrive/articles/Geometry%20Concepts%20part%201:%20Basic%20Concepts#PolygonArea
    // slightly tweaking it, since we're adding a half-width "wrapper" around that shape.

    let mut digger = Vector { row: 0, col: 0 };
    let mut corners: Vec<Vector> = Vec::new();

    let mut wrapper_area = 1;
    for (direction, count) in instructions {
        wrapper_area += count;

        digger = digger.add(&direction.mul(*count));
        corners.push(digger);
    }

    wrapper_area /= 2;
    wrapper_area += 1;

    let mut internal_area = 0;
    for (i, corner) in corners.iter().enumerate() {
        let j = (i + 1) % corners.len();
        let next_corner = corners[j];

        internal_area += (corner.row + 3) * (next_corner.col + 3);
        internal_area -= (corner.col + 3) * (next_corner.row + 3);
    }

    internal_area = internal_area.abs() / 2;
    internal_area + wrapper_area
}

fn build_instructions_part1(lines: &Vec<String>) -> Vec<(Vector, isize)> {
    lines
        .iter()
        .map(|line| {
            let (dir, rest) = line.split_once(' ').unwrap();
            let (count, _) = rest.split_once(' ').unwrap();

            let direction = match dir {
                "R" => Vector { row: 0, col: 1 },
                "L" => Vector { row: 0, col: -1 },
                "U" => Vector { row: -1, col: 0 },
                "D" => Vector { row: 1, col: 0 },
                _ => {
                    panic!("Unexpected direction '{dir}'")
                }
            };

            (direction, count.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn build_instructions_part2(lines: &Vec<String>) -> Vec<(Vector, isize)> {
    lines
        .iter()
        .map(|line| {
            let (_, hex) = line.split_once('#').unwrap();

            let count = isize::from_str_radix(&hex[0..5], 16).unwrap();
            let dir_char = hex.chars().nth(5).unwrap();
            let direction = match dir_char {
                '0' => Vector { row: 0, col: 1 },
                '2' => Vector { row: 0, col: -1 },
                '3' => Vector { row: -1, col: 0 },
                '1' => Vector { row: 1, col: 0 },
                _ => {
                    panic!("Unexpected direction '{dir_char}'")
                }
            };

            (direction, count)
        })
        .collect::<Vec<_>>()
}
