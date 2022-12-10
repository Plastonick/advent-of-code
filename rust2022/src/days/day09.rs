use crate::common::get_lines;
use std::cmp::max;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;
use std::{thread, time};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait DistanceTo {
    fn distance_to(&self, target: Point) -> Point;
}

impl DistanceTo for Point {
    fn distance_to(&self, target: Point) -> Point {
        Point {
            x: self.x - target.x,
            y: self.y - target.y,
        }
    }
}

pub fn run(visual: bool) {
    let lines = get_lines("day09");

    let mut head_knot = Point { x: 0, y: 0 };
    let mut tail_knots = [Point { x: 0, y: 0 }; 9];
    let mut first_tail_positions = HashSet::new();
    let mut last_tail_positions = HashSet::new();
    let zero_vector = Point { x: 0, y: 0 };
    let commands = lines.iter().map(map_command);

    for (direction, magnitude) in commands {
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

            if visual {
                print_state(&head_knot, &tail_knots);
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

fn map_command(command: &String) -> (Point, isize) {
    let (direction, magnitude) = command.split_once(' ').unwrap();
    let direction_vector = match direction {
        "U" => Point { x: 0, y: 1 },
        "D" => Point { x: 0, y: -1 },
        "L" => Point { x: -1, y: 0 },
        "R" => Point { x: 1, y: 0 },
        _ => Point { x: 1, y: 0 },
    };

    (direction_vector, magnitude.parse::<isize>().unwrap())
}

fn move_head(head: Point, tail: Point, direction: Point) -> (Point, Point) {
    let new_head = head + direction;
    let head_tail_distance = new_head.distance_to(tail);

    if head_tail_distance.x.abs() <= 1 && head_tail_distance.y.abs() <= 1 {
        (new_head, tail)
    } else {
        // The tail should move a single unit in the direction of the head, in both dimensions
        let tail_move = Point {
            x: head_tail_distance.x / max(head_tail_distance.x.abs(), 1),
            y: head_tail_distance.y / max(head_tail_distance.y.abs(), 1),
        };

        (new_head, tail + tail_move)
    }
}

fn print_state(head: &Point, tails: &[Point; 9]) {
    let mut screen = [[' '; 21]; 21];

    for i in 0..21 {
        for j in 0..21 {
            if (i - head.x) % 10 == 0 || (j - head.y) % 10 == 0 {
                screen[i as usize][j as usize] = '.';
            }
        }
    }

    for i in 1..tails.len() {
        let node = tails[i - 1];
        let next = tails[i];
        let x = (10 + node.x - head.x) as usize;
        let y = (10 + node.y - head.y) as usize;

        let direction = Point {
            x: next.x - node.x,
            y: next.y - node.y,
        };

        screen[20 - x][20 - y] = match (direction.x, direction.y) {
            (1, 1) | (-1, -1) => '\\',
            (-1, 1) | (1, -1) => '/',
            (0, 1) | (0, -1) => '-',
            (1, 0) | (-1, 0) => '|',
            (0, 0) | _ => '+',
        };
    }

    // centre the screen on the head
    screen[10][10] = 'o';

    for line in screen {
        for chararacter in line {
            print!("{}", chararacter);
        }
        println!();
    }

    println!();
    println!();
    thread::sleep(time::Duration::from_millis(30));
}
