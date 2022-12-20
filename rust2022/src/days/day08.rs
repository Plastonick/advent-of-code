use std::cmp::max;

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    let lines = get_lines("day08");

    let canopy: Vec<Vec<u8>> = lines
        .iter()
        .map(|x| x.as_bytes().iter().map(|b| 1 + b - ('0' as u8)).collect())
        .collect();

    let mut visible = 0;
    let mut max_scenic_score = 0;

    for (i, row) in canopy.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            visible += if is_visible((i, j), &canopy) { 1 } else { 0 };
            max_scenic_score = max(max_scenic_score, scenic_score((i, j), &canopy));
        }
    }

    if !args.no_answers {
        println!(
            "Day 8, Part 1: There are {} trees visible from outside",
            visible
        );
        println!(
            "Day 8, Part 2: The highest scenic score available is {}",
            max_scenic_score
        );
    }

    ("".to_string(), "".to_string())
}

fn is_visible(pos: (usize, usize), canopy: &Vec<Vec<u8>>) -> bool {
    let vectors: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for vector in vectors {
        if is_visible_from_vector(pos, vector, &canopy) {
            return true;
        }
    }

    false
}

fn is_visible_from_vector(
    initial: (usize, usize),
    vector: (isize, isize),
    canopy: &Vec<Vec<u8>>,
) -> bool {
    let target_height = canopy[initial.0][initial.1];
    let mut running_pos = (initial.0 as isize + vector.0, initial.1 as isize + vector.1);

    // make sure we're not about to look out of bounds
    while running_pos.0 >= 0
        && running_pos.1 >= 0
        && running_pos.0 < canopy.len() as isize
        && running_pos.1 < canopy[running_pos.0 as usize].len() as isize
    {
        let height_at_pos = canopy[running_pos.0 as usize][running_pos.1 as usize];

        if height_at_pos >= target_height {
            return false;
        }

        running_pos = (running_pos.0 + vector.0, running_pos.1 + vector.1)
    }

    true
}

fn scenic_score(pos: (usize, usize), canopy: &Vec<Vec<u8>>) -> usize {
    let vectors: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut scenic_score = 1;

    for vector in vectors {
        scenic_score *= scenic_score_from_vector(pos, vector, &canopy);
    }

    scenic_score
}

fn scenic_score_from_vector(
    initial: (usize, usize),
    vector: (isize, isize),
    canopy: &Vec<Vec<u8>>,
) -> usize {
    let target_height = canopy[initial.0][initial.1];
    let mut running_pos = (initial.0 as isize + vector.0, initial.1 as isize + vector.1);

    let mut seen = 0;

    // make sure we're not about to look out of bounds
    while running_pos.0 >= 0
        && running_pos.1 >= 0
        && running_pos.0 < canopy.len() as isize
        && running_pos.1 < canopy[running_pos.0 as usize].len() as isize
    {
        let height_at_pos = canopy[running_pos.0 as usize][running_pos.1 as usize];
        seen += 1;

        if height_at_pos >= target_height {
            return seen;
        }

        running_pos = (running_pos.0 + vector.0, running_pos.1 + vector.1)
    }

    seen
}
