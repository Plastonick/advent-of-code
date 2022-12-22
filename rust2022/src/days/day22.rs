use std::{cmp::max, collections::HashMap};

use crate::{common::get_file_contents, Args};

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Player {
    position: (i32, i32),
    direction: (i32, i32),
}

#[derive(Debug)]
struct Map {
    blocks: HashMap<(i32, i32), Block>,
    bounds: (i32, i32),
}

pub fn run(args: &Args) -> (String, String) {
    let file = if args.test {
        get_file_contents("day22-test")
    } else {
        get_file_contents("day22")
    };

    let (map_str, command_str) = file.split_once("\n\n").unwrap();
    let mut map = HashMap::new();
    let mut commands = Vec::new();
    let mut command = String::new();
    let mut start_point = None;

    for ch in command_str.chars() {
        match ch {
            '0'..='9' => command.push(ch),
            'R' | 'L' => {
                if !command.is_empty() {
                    commands.push(command);
                }
                command = String::new();
                commands.push(ch.to_string());
            }
            _ => {
                if !command.is_empty() {
                    commands.push(command);
                }
                command = String::new();
            }
        }
    }

    // let num_lines = map_str.lines().count();
    // // let num_lines = map_str.lines().collect::<Vec<_>>().len();

    let mut max_x = 0;
    let mut max_y = 0;

    for (row, line) in map_str.lines().enumerate() {
        max_y = max(max_y, row as i32);

        for (column, character) in line.chars().enumerate() {
            let point = (row as i32, column as i32);

            max_x = max(max_x, column as i32);

            match character {
                '.' => {
                    if start_point.is_none() {
                        start_point = Some(point);
                    }
                    map.insert(point, Block::Empty);
                }
                '#' => {
                    if start_point.is_none() {
                        start_point = Some(point);
                    }
                    map.insert(point, Block::Wall);
                }
                _ => { /* do nothing */ }
            }
        }
    }

    let map = Map {
        blocks: map,
        bounds: (max_y, max_x),
    };

    let mut current = Player {
        position: start_point.unwrap(),
        direction: (0, 1),
    };

    let rotate_right = String::from("R");
    let rotate_left = String::from("L");

    dbg!(&commands);

    // println!(
    //     "Moving from... ({}, {})",
    //     current.position.0 + 1,
    //     current.position.1 + 1
    // );

    for command in commands {
        if command == rotate_right {
            current = Player {
                direction: (current.direction.1, -current.direction.0),
                ..current
            };
        } else if command == rotate_left {
            current = Player {
                direction: (-current.direction.1, current.direction.0),
                ..current
            };
        } else {
            current = move_units(current, command.parse::<i32>().unwrap(), &map)
        }

        // dbg!(&current);
    }
    let part1 = password(&current);

    if !args.no_answers {
        println!("Day 22, Part 1: The password is {}", part1);
    }

    (part1.to_string(), "".to_string())
}

fn move_units(player: Player, units: i32, map: &Map) -> Player {
    let mut pos = player.position;

    for _ in 0..units {
        let next = next_pos(pos, player.direction, map);

        // println!("Moving to... ({}, {})", pos.0 + 1, pos.1 + 1);

        let block = map.blocks.get(&next).unwrap();

        if block == &Block::Wall {
            // We've gone as far as we can go! Stop.
            break;
        } else {
            // else, next position is empty, move and keep going!
            pos = next;
        }
    }

    // println!("Stopped ({}, {})", pos.0 + 1, pos.1 + 1);

    Player {
        position: pos,
        ..player
    }
}

fn next_pos(pos: (i32, i32), direction: (i32, i32), map: &Map) -> (i32, i32) {
    let mut pos = (
        (pos.0 + direction.0).rem_euclid(map.bounds.0),
        (pos.1 + direction.1).rem_euclid(map.bounds.1),
    );

    while let None = map.blocks.get(&pos) {
        pos = (
            (pos.0 + direction.0).rem_euclid(map.bounds.0),
            (pos.1 + direction.1).rem_euclid(map.bounds.1),
        )
    }

    pos
}

fn password(player: &Player) -> i32 {
    let facing = match player.direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => {
            panic!("Unexpected direction!")
        }
    };

    ((player.position.0 + 1) * 1000) + ((player.position.1 + 1) * 4) + facing
}
