use std::cmp::max;

use crate::common::get_file_contents;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub fn run(visual: bool) {
    let file = get_file_contents("day17");
    let directions = file
        .as_bytes()
        .iter()
        .map(|x| {
            if *x as char == '<' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect::<Vec<_>>();

    // Blocks
    //
    // ####
    //
    // .#.
    // ###
    // .#.
    //
    // ..#
    // ..#
    // ###
    //
    // #
    // #
    // #
    // #
    //
    // ##
    // ##

    let blocks = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut max_height = 0;

    let mut block_index = 0;
    let mut direction_index = 0;
    let mut board: Vec<(isize, isize)> = Vec::new();
    // let total_moves = 1_000_000_000_000;
    let total_moves = 3000;
    let part_1_limit = 2022;
    let mut part_1_solution = 0;

    while block_index < total_moves {
        if block_index == 2022 {
            part_1_solution = max_height;
        }

        let block = &blocks[block_index % blocks.len()];

        let mut height = max_height + 3;
        let mut left = 2;

        loop {
            // apply wind
            let direction = match &directions[direction_index] {
                Direction::Left => -1,
                Direction::Right => 1,
            };

            direction_index = (direction_index + 1) % directions.len();

            // move the block with the wind!
            left += direction;

            // if the block can't exist with this new left_offset, revert the offset
            if !can_exist(height, left, block, &board) {
                left -= direction;
            }

            // move block down
            height -= 1;

            // if the block can't exist lower down, revert the height drop, freeze the block there, and terminate
            if !can_exist(height, left, block, &board) {
                height += 1;
                // add the block to the board, and re-calculate the max height
                for point in block {
                    let x = point.0 + left;
                    let y = point.1 + height;

                    max_height = max(max_height, y + 1);
                    board.push((x, y));
                }

                break;
            }
        }

        block_index += 1;
    }

    if visual {
        print(&board);
    }

    println!(
        "Day 17, Part 1: The tetris board is {} units high after {} moves",
        part_1_solution, part_1_limit
    );
}

fn can_exist(
    height: isize,
    left: isize,
    block: &Vec<(isize, isize)>,
    board: &Vec<(isize, isize)>,
) -> bool {
    if height < 0 || left < 0 || left > 6 {
        return false;
    }

    for point in block {
        let x = point.0 + left;
        let y = point.1 + height;

        if x < 0 || x >= 7 {
            return false;
        }

        if board.contains(&(x, y)) {
            return false;
        }
    }

    true
}

fn print(board: &Vec<(isize, isize)>) {
    let mut height = 0;
    let width = 7;

    for (_, y) in board {
        height = max(height, y.to_owned());
    }

    for y in 0..=height {
        print!("|");
        // print it top to bottom
        let yinv = height - y;
        for x in 0..width {
            if board.contains(&(x, yinv)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("|");
        println!();
    }

    println!("+-------+");
    println!();
    println!();
    println!();
}
