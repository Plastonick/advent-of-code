use crate::common::get_lines;

pub fn run() {
    let lines = get_lines("day08");

    let canopy: Vec<Vec<u8>> = lines
        .iter()
        .map(|x| x.as_bytes().iter().map(|b| 1 + b - ('0' as u8)).collect())
        .collect();

    let mut visible = 0;

    for (i, row) in canopy.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            visible += if is_visible((i, j), &canopy) { 1 } else { 0 };
        }
    }

    println!(
        "Day 8, Part 1: There are {} trees visible from outside",
        visible
    );
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
