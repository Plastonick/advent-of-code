use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use crate::common::get_file_contents;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub fn run(_: bool) {
    let file = get_file_contents("day17-test");
    let directions = file
        .as_bytes()
        .iter()
        .map(|x| match *x as char {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => {
                println!("Uh oh! This isn't a direction...");
                panic!()
            }
        })
        .collect::<Vec<_>>();

    // Blocks

    // | #### |

    // |  #  |
    // | ### |
    // |  #  |

    // |   # |
    // |   # |
    // | ### |

    // | # |
    // | # |
    // | # |
    // | # |

    // | ## |
    // | ## |

    let blocks = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let limits: [usize; 2] = [2022, 1_000_000_000_000];
    for (i, limit) in limits.iter().enumerate() {
        let height = height_after_blocks(limit.to_owned(), &blocks, &directions);

        println!(
            "Day 17, Part {}: The tetris board is {} units high after {} moves",
            i + 1,
            height,
            limit
        );
    }
}

fn height_after_blocks(
    limit: usize,
    blocks: &[Vec<(isize, isize)>; 5],
    directions: &Vec<Direction>,
) -> isize {
    let mut block_num = 0;
    let mut direction_index = 0;
    let mut board: HashSet<(isize, isize)> = HashSet::new();
    let mut max_height = 0;
    let mut cache: HashMap<(usize, usize), (isize, usize)> = HashMap::new();
    let mut cycle_detected = false;

    while block_num < limit {
        let key = (block_num % blocks.len(), direction_index);

        // go until we've detected a loop
        if !cycle_detected {
            if let Some((height, prev_block_num)) = cache.get(&key) {
                cycle_detected = true;
                let cycle_height = max_height - height.to_owned();
                let cycle_length = block_num - prev_block_num;
                let old_max_height = max_height;

                // increase the loop a bunch!
                let cycles_remaining: usize = ((limit - block_num) / cycle_length) - 1;
                max_height += cycles_remaining as isize * cycle_height;
                block_num += cycles_remaining * cycle_length;

                // copy the last cycle
                for y in 0..cycle_height {
                    for x in 0..7 {
                        let old_key = (x, old_max_height - y);
                        let new_key = (x, max_height - y);

                        if board.contains(&old_key) {
                            board.insert(new_key);
                        }
                    }
                }
            }
        }

        (direction_index, max_height) = iterate(
            block_num,
            direction_index,
            max_height,
            &blocks,
            &directions,
            &mut board,
        );

        cache.insert(key, (max_height, block_num));

        block_num += 1;
    }

    max_height
}

fn iterate(
    block_idx: usize,
    direction_index: usize,
    max_height: isize,
    blocks: &[Vec<(isize, isize)>; 5],
    directions: &Vec<Direction>,
    board: &mut HashSet<(isize, isize)>,
) -> (usize, isize) {
    let block = &blocks[block_idx % blocks.len()];

    let mut out_max_height = max_height;
    let mut out_direction_index = direction_index;
    let mut height = max_height + 3;
    let mut left = 2;

    loop {
        // apply wind
        let direction = match &directions[out_direction_index] {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        out_direction_index = (out_direction_index + 1) % directions.len();

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

                out_max_height = max(out_max_height, y + 1);
                board.insert((x, y));
            }

            break;
        }
    }

    (out_direction_index, out_max_height)
}

fn can_exist(
    height: isize,
    left: isize,
    block: &Vec<(isize, isize)>,
    board: &HashSet<(isize, isize)>,
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

fn print(board: &HashSet<(isize, isize)>) {
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
