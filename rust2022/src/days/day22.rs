use std::collections::HashMap;
use std::hash::Hash;
use std::{thread, time};

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

enum Strategy {
    Net,
    Cube,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_vector(direction: (i32, i32)) -> Direction {
        match direction {
            (0, 1) => Direction::Right,
            (0, -1) => Direction::Left,
            (1, 0) => Direction::Down,
            (-1, 0) => Direction::Up,
            _ => {
                panic!("Unexpected direction! ({}, {})", direction.0, direction.1)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Face {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
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

    let num_rows = map_str.lines().count() as i32;
    let num_cols = map_str.lines().next().unwrap().chars().count() as i32;

    for (row, line) in map_str.lines().enumerate() {
        for (column, ch) in line.chars().enumerate() {
            let point = (row as i32, column as i32);

            match ch {
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
                ' ' => { /* do nothing */ }
                _ => {
                    panic!("What is this!? '{}'", ch)
                }
            }
        }
    }

    let map = Map {
        blocks: map,
        bounds: (num_rows, num_cols),
    };

    let start = Player {
        position: start_point.unwrap(),
        direction: (0, 1),
    };

    dbg!(start_point.unwrap());

    // let final_p1 = move_player(&start, &commands, &map, &Strategy::Net);
    let final_p1 = Player { ..start };
    let final_p2 = move_player(&start, &commands, &map, &Strategy::Cube);

    let part1 = password(&final_p1);
    let part2 = password(&final_p2);

    if !args.no_answers {
        println!("Day 22, Part 1: For the net, the password is {}", part1);
        println!("Day 22, Part 2: For the cube, the password is {}", part2);
    }

    (part1.to_string(), part2.to_string())
}

fn move_player(current: &Player, commands: &Vec<String>, map: &Map, strategy: &Strategy) -> Player {
    let mut current = Player { ..*current };

    let rotate_right = &String::from("R");
    let rotate_left = &String::from("L");

    // println!(
    //     "Moving from... ({}, {})",
    //     current.position.0 + 1,
    //     current.position.1 + 1
    // );

    let mut path = HashMap::new();

    for command in commands {
        if command == rotate_right {
            current = Player {
                direction: (current.direction.1, -current.direction.0),
                ..current
            };
            // println!("Rotate RIGHT");
        } else if command == rotate_left {
            current = Player {
                direction: (-current.direction.1, current.direction.0),
                ..current
            };

            // println!("Rotate LEFT");
        } else {
            let units = command.parse::<i32>().unwrap();

            // println!("Units: {}", units);
            current = move_units(current, units, &map, strategy, &mut path)
        }
        let dir_char = match Direction::from_vector(current.direction) {
            Direction::Down => 'v',
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Left => '<',
        };

        path.insert(current.position, dir_char);

        // dbg!(&current);
    }

    current
}

fn move_units(
    player: Player,
    units: i32,
    map: &Map,
    strategy: &Strategy,
    path: &mut HashMap<(i32, i32), char>,
) -> Player {
    let mut player = player;

    for _ in 0..units {
        let next = next_pos(Player { ..player }, map, strategy);

        // println!("Moving to... ({}, {})", pos.0 + 1, pos.1 + 1);

        let block = map.blocks.get(&next.position).unwrap();

        if block == &Block::Wall {
            // We've gone as far as we can go! Stop.
            break;
        } else {
            // else, next position is empty, move and keep going!
            player = next;
        }

        let dir_char = match Direction::from_vector(player.direction) {
            Direction::Down => 'v',
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Left => '<',
        };

        path.insert(player.position, dir_char);

        // _print_map(map, &path);
    }

    // println!("Stopped ({}, {})", pos.0 + 1, pos.1 + 1);

    Player { ..player }
}

fn next_pos(player: Player, map: &Map, strategy: &Strategy) -> Player {
    let mut player = player;

    loop {
        player = match strategy {
            Strategy::Net => add_net(player, map),
            Strategy::Cube => add_cube(player, map),
        };

        if map.blocks.contains_key(&player.position) {
            break;
        }
    }

    player
}

fn add_net(player: Player, map: &Map) -> Player {
    Player {
        position: (
            (player.position.0 + player.direction.0).rem_euclid(map.bounds.0),
            (player.position.1 + player.direction.1).rem_euclid(map.bounds.1),
        ),
        ..player
    }
}

fn add_cube(player: Player, map: &Map) -> Player {
    let new_pos = (
        player.position.0 + player.direction.0,
        player.position.1 + player.direction.1,
    );

    if map.blocks.contains_key(&new_pos) {
        // println!("Found a place");

        return Player {
            position: new_pos,
            ..player
        };
    }

    let face = get_face(player.position);

    let right = (0, 1);
    let left = (0, -1);
    let down = (1, 0);
    let up = (-1, 0);

    let row_group = player.position.0 / 50;
    let col_group = player.position.1 / 50;

    let face_top_left = (row_group * 50, col_group * 50);

    let face_1 = (0, 50);
    let face_2 = (0, 100);
    let face_3 = (50, 50);
    let face_4 = (100, 0);
    let face_5 = (100, 50);
    let face_6 = (150, 0);

    let direction = Direction::from_vector(player.direction);

    println!(
        "Moving from face ({}, {}), pos ({}, {}), dir ({}, {})",
        face_top_left.0,
        face_top_left.1,
        new_pos.0,
        new_pos.1,
        player.direction.0,
        player.direction.1,
    );

    // We're going off the edge of the net
    // need to find _where_ it should go, then also permute its direction and position
    let (target_top_left, target_direction) = match (direction, face) {
        (Direction::Left, Face::One) => (face_4, right),
        (Direction::Up, Face::One) => (face_6, right),
        (Direction::Up, Face::Two) => (face_6, up),
        (Direction::Right, Face::Two) => (face_5, left),
        (Direction::Down, Face::Two) => (face_3, left),
        (Direction::Left, Face::Three) => (face_4, down),
        (Direction::Right, Face::Three) => (face_2, up),
        (Direction::Up, Face::Four) => (face_3, right),
        (Direction::Left, Face::Four) => (face_1, right),
        (Direction::Right, Face::Five) => (face_2, left),
        (Direction::Down, Face::Five) => (face_6, left),
        (Direction::Left, Face::Six) => (face_1, down),
        (Direction::Down, Face::Six) => (face_2, down),
        (Direction::Right, Face::Six) => (face_5, up),
        _ => {
            // dbg!(&direction, &face, &new_pos);
            panic!("Unexpected combination!")
        }
    };

    let new_pos = warp_vector(
        &Player { ..player }, // pretend like we're still in the old face...
        face_top_left,
        target_direction,
        target_top_left,
    );

    println!(
        "...to face  ({}, {}), pos ({}, {}), dir ({}, {})",
        target_top_left.0,
        target_top_left.1,
        new_pos.0,
        new_pos.1,
        target_direction.0,
        target_direction.1,
    );

    // dbg!(new_pos);
    println!();

    Player {
        position: new_pos,
        direction: target_direction,
    }
}

fn warp_vector(
    player: &Player,
    from_top_left: (i32, i32),
    target_direction: (i32, i32),
    to_top_left: (i32, i32),
) -> (i32, i32) {
    let half_side = 25;

    // centre to the origin, based on the original face
    let centered = (
        player.position.0 - from_top_left.0 - half_side,
        player.position.1 - from_top_left.1 - half_side,
    );

    println!(
        "Centred ({}, {}) to ({}, {})",
        player.position.0, player.position.1, centered.0, centered.1
    );

    // now reflect in y=-x
    let mut reflected = (-centered.1, -centered.0);

    // reflect the original direction too, so we can use it as an anchor
    let original_direction = (-player.direction.1, -player.direction.0);
    let mut target_direction = target_direction;

    // rotate our original direction until it matches the target direction
    while original_direction != target_direction {
        target_direction = rotate_90(target_direction);
        reflected = rotate_90(reflected);
    }

    // translate our original with respect to its _new_ face
    (
        reflected.0 + to_top_left.0 + half_side,
        reflected.1 + to_top_left.1 + half_side,
    )
}

fn rotate_90(vector: (i32, i32)) -> (i32, i32) {
    (-vector.1, vector.0)
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

    // println!(
    //     "Facing is {} for ({}, {})",
    //     facing, player.direction.0, player.direction.1
    // );

    ((player.position.0 + 1) * 1000) + ((player.position.1 + 1) * 4) + facing
}

fn _print_map(map: &Map, path: &HashMap<(i32, i32), char>) {
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();

    for r in 0..map.bounds.0 {
        for c in 0..map.bounds.1 {
            let point = (r, c);

            if let Some(step) = path.get(&point) {
                print!("{}", step);
            } else if let Some(block) = map.blocks.get(&point) {
                print!(
                    "{}",
                    match block {
                        Block::Empty => ' ',
                        Block::Wall => '#',
                    }
                );
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    println!();
    thread::sleep(time::Duration::from_millis(250));

    // println!("Bounds: {} rows and {} columns", map.bounds.0, map.bounds.1)
}

fn get_face(pos: (i32, i32)) -> Face {
    let row_group = pos.0 / 50;
    let col_group = pos.1 / 50;

    match (row_group, col_group) {
        (0, 1) => Face::One,
        (0, 2) => Face::Two,
        (1, 1) => Face::Three,
        (2, 0) => Face::Four,
        (2, 1) => Face::Five,
        (3, 0) => Face::Six,
        _ => {
            panic!("Unexpected face! ({}, {})", pos.0, pos.1)
        }
    }
}

// fn two_to_three_d(pos: (i32, i32)) -> (i32, i32, i32) {
//     // which cube face are we on?
//     let face = if 0 <= pos.0 && pos.0 < 50 {
//         if pos.1 >= 100 {
//             1
//         } else {
//             2
//         }
//     } else if 50 <= pos.0 && pos.0 < 100 {
//         3
//     } else if 100 <= pos.0 && pos.0 < 150 {
//         if pos.1 >= 50 {
//             4
//         } else {
//             5
//         }
//     } else {
//         6
//     };

//     // map the 2d pos on that cube face to a 3d position
//     let point = match face {
//         1 => (pos.0, pos.1 - 50, 0),
//         2 => (0, 0, pos.0 - 100),
//         3 => (50, pos.1 - 50, pos.0 - 50),
//         4 => (0, 0, 0),
//         5 => (0, 0, 0),
//         6 => (0, 0, 0),
//         _ => {
//             panic!("Unexpected face!")
//         }
//     };

//     // make sure we have a valid 3d point
//     assert!(point.0 >= 0);
//     assert!(point.1 >= 0);
//     assert!(point.2 >= 0);
//     assert!(point.0 < 50);
//     assert!(point.1 < 50);
//     assert!(point.2 < 50);

//     point
// }

// fn three_to_two_d(pos: (i32, i32, i32)) -> (i32, i32) {
//     (0, 0)
// }
