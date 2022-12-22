use std::collections::HashMap;

use crate::{common::get_file_contents, Args};

#[derive(Debug)]
enum Block {
    Empty,
    Wall,
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

    for (row, line) in map_str.lines().enumerate() {
        for (column, character) in line.chars().enumerate() {
            let point = (row + 1, column + 1);

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

    dbg!(map, start_point.unwrap());

    let mut current = (start_point.unwrap(), (0, 1));
    let rotate_right = String::from("R");
    let rotate_left = String::from("L");

    for command in commands {
        match command {
            rotate_right => current = (current.0, current.1),
            rotate_left => {}
            _ => {}
        }
    }

    ("".to_string(), "".to_string())
}

fn rotate(direction: (i32, i32), rotation: &String) -> (i32, i32) {
    let rotate_right = String::from("R");
    let rotate_left = String::from("L");

    match rotation {
        rotate_right => (1, 0),
        rotate_left => (1, 0),
        _ => (0, 0),
    }
}
