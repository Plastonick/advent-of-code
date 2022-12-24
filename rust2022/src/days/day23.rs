use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use crate::{common::get_lines, Args};

type Point = (i32, i32);

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day23-test")
    } else {
        get_lines("day23")
    };

    let mut elves = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().into_iter().enumerate() {
            if ch == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    let mut part1 = 0;
    let part2;
    let mut i = 0;
    loop {
        let optional_elves = process_round(elves, i);

        if optional_elves.is_some() {
            if i == 10 {
                let elf_area = get_area(&optional_elves.clone().unwrap());
                part1 = elf_area as usize - optional_elves.clone().unwrap().len();
            }

            i += 1;
        } else {
            part2 = i + 1;
            break;
        }

        elves = optional_elves.unwrap();
    }

    if !args.no_answers {
        println!("Day 23, Part 1: The empty elf space is {}", part1);
        println!("Day 23, Part 1: The first inactive round is {}", part2);
    }

    (part1.to_string(), part2.to_string())
}

fn _print(elves: &HashSet<Point>) {
    let (minimums, maximums) = get_bounds(elves);

    println!("----------------------");

    for i in minimums.0..=maximums.0 {
        for j in minimums.1..=maximums.1 {
            let point = (i, j);

            if elves.contains(&point) {
                print!("#")
            } else {
                print!(".")
            }
        }

        print!("\n")
    }

    println!("----------------------");
    println!();
}

fn get_area(elves: &HashSet<Point>) -> i32 {
    let (minimums, maximums) = get_bounds(elves);

    (1 + maximums.0 - minimums.0) * (1 + maximums.1 - minimums.1)
}

fn get_bounds(elves: &HashSet<Point>) -> (Point, Point) {
    let mut minimums = (i32::MAX, i32::MAX);
    let mut maximums = (i32::MIN, i32::MIN);

    for elf in elves {
        minimums = (min(minimums.0, elf.0), min(minimums.1, elf.1));
        maximums = (max(maximums.0, elf.0), max(maximums.1, elf.1));
    }

    (minimums, maximums)
}

fn process_round(elves: HashSet<Point>, round: usize) -> Option<HashSet<Point>> {
    let mut out_elves = elves.clone();

    // round 1, propose moves!
    let mut proposed: HashMap<Point, Vec<Point>> = HashMap::new();
    for elf in &elves {
        // check north
        if let Some(proposal) = propose_move(elf, &elves, round) {
            let mut proposals = if let Some(existing) = proposed.get(&proposal) {
                existing.clone()
            } else {
                Vec::new()
            };

            proposals.push(*elf);
            proposed.insert(proposal, proposals);
        }
    }

    if proposed.is_empty() {
        return None;
    }

    // round 2, move if possible
    for (proposal, elves) in proposed {
        if elves.len() == 1 {
            // a single proposal for this place, move the elf!
            out_elves.insert(proposal);
            out_elves.remove(&elves[0]);
        }
    }

    Some(out_elves)
}

fn propose_move(point: &Point, elves: &HashSet<Point>, index: usize) -> Option<Point> {
    // does it need to move?
    if is_isolated(point, elves) {
        return None;
    }

    let moves = [move_north, move_south, move_west, move_east];

    for i in 0..4 {
        // try move
        if let Some(proposal) = moves[(i + index) % 4](point, elves) {
            return Some(proposal);
        }
    }

    None
}

fn is_isolated(point: &Point, elves: &HashSet<Point>) -> bool {
    for neighbour in get_neighbours(point) {
        if elves.contains(&neighbour) {
            return false;
        }
    }

    return true;
}

fn get_neighbours(point: &Point) -> Vec<Point> {
    let mut neighbours = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            neighbours.push((point.0 + i, point.1 + j));
        }
    }

    neighbours
}

fn move_north(point: &Point, elves: &HashSet<Point>) -> Option<Point> {
    let north = point.0 - 1;
    let west = point.1 - 1;
    let east = point.1 + 1;

    if !(elves.contains(&(north, west))
        || elves.contains(&(north, point.1))
        || elves.contains(&(north, east)))
    {
        return Some((north, point.1));
    }

    None
}

fn move_south(point: &Point, elves: &HashSet<Point>) -> Option<Point> {
    let south = point.0 + 1;
    let west = point.1 - 1;
    let east = point.1 + 1;

    if !(elves.contains(&(south, west))
        || elves.contains(&(south, point.1))
        || elves.contains(&(south, east)))
    {
        return Some((south, point.1));
    }

    None
}

fn move_west(point: &Point, elves: &HashSet<Point>) -> Option<Point> {
    let north = point.0 - 1;
    let south = point.0 + 1;
    let west = point.1 - 1;

    if !(elves.contains(&(north, west))
        || elves.contains(&(point.0, west))
        || elves.contains(&(south, west)))
    {
        return Some((point.0, west));
    }

    None
}

fn move_east(point: &Point, elves: &HashSet<Point>) -> Option<Point> {
    let north = point.0 - 1;
    let south = point.0 + 1;
    let east = point.1 + 1;

    if !(elves.contains(&(north, east))
        || elves.contains(&(point.0, east))
        || elves.contains(&(south, east)))
    {
        return Some((point.0, east));
    }

    None
}
