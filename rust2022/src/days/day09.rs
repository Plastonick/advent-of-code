use crate::common::get_lines;
use std::cmp::max;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vector {
    x: isize,
    y: isize,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait DistanceTo {
    fn distance_to(&self, target: Vector) -> Vector;
}

impl DistanceTo for Vector {
    fn distance_to(&self, target: Vector) -> Vector {
        Vector {
            x: self.x - target.x,
            y: self.y - target.y,
        }
    }
}

pub fn run() {
    let lines = get_lines("day09");

    let mut head_knot = Vector { x: 0, y: 0 };
    let mut tail_knots = [Vector { x: 0, y: 0 }; 9];
    let mut first_tail_positions = HashSet::new();
    let mut last_tail_positions = HashSet::new();
    let zero_vector = Vector { x: 0, y: 0 };

    for (direction, magnitude) in lines.iter().map(map_command) {
        for _ in 0..magnitude {
            let (lead_knot, trailing_knot) = move_head(head_knot, tail_knots[0], direction);

            head_knot = lead_knot;
            tail_knots[0] = trailing_knot;

            for i in 1..tail_knots.len() {
                let (lead_knot, trailing_knot) =
                    move_head(tail_knots[i - 1], tail_knots[i], zero_vector);

                tail_knots[i - 1] = lead_knot;
                tail_knots[i] = trailing_knot;
            }

            // record tail position
            first_tail_positions.insert(tail_knots[0]);
            last_tail_positions.insert(tail_knots[tail_knots.len() - 1]);
        }
    }

    println!(
        "Day 9, Part 1: The first tail visits {} unique position",
        first_tail_positions.len()
    );

    println!(
        "Day 9, Part 2: The last tail (of 9) visits {} unique position",
        last_tail_positions.len()
    )
}

fn map_command(command: &String) -> (Vector, isize) {
    let (direction, magnitude) = command.split_once(' ').unwrap();
    let direction_vector = match direction {
        "U" => Vector { x: 0, y: 1 },
        "D" => Vector { x: 0, y: -1 },
        "L" => Vector { x: -1, y: 0 },
        "R" => Vector { x: 1, y: 0 },
        _ => Vector { x: 1, y: 0 },
    };

    (direction_vector, magnitude.parse::<isize>().unwrap())
}

fn move_head(head: Vector, tail: Vector, direction: Vector) -> (Vector, Vector) {
    let new_head = head + direction;
    let head_tail_distance = new_head.distance_to(tail);

    if head_tail_distance.x.abs() <= 1 && head_tail_distance.y.abs() <= 1 {
        (new_head, tail)
    } else {
        // The tail should move a single unit in the direction of the head, in both dimensions
        let tail_move = Vector {
            x: head_tail_distance.x / max(head_tail_distance.x.abs(), 1),
            y: head_tail_distance.y / max(head_tail_distance.y.abs(), 1),
        };

        (new_head, tail + tail_move)
    }
}
