use std::collections::HashMap;
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

type Position = (i32, i32);

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

    let final_p1 = move_player(&start, &commands, &map, &Strategy::Net);
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

    let mut path = HashMap::new();

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
            let units = command.parse::<i32>().unwrap();

            current = move_units(current, units, &map, strategy, &mut path)
        }
        let dir_char = match Direction::from_vector(current.direction) {
            Direction::Down => 'v',
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Left => '<',
        };

        path.insert(current.position, dir_char);
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
    }

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

    // if we don't need to wrap at all, trivially return the new posiiton
    if map.blocks.contains_key(&new_pos) {
        return Player {
            position: new_pos,
            ..player
        };
    }

    // now, find out which face we're currently at, and which face we are going towards
    let (target_top_left, target_direction) = get_target(&player);
    let face_top_left = get_face_top_left(&player);

    // this is our current position warped to the new face, we still need to move in the new direction!
    let warped_position = warp_vector(
        &Player { ..player }, // pretend like we're still in the old face...
        face_top_left,
        target_direction,
        target_top_left,
    );

    let new_pos = (warped_position.0, warped_position.1);

    Player {
        position: new_pos,
        direction: target_direction,
    }
}

fn get_face_top_left(player: &Player) -> Position {
    let row_group = player.position.0 / 50;
    let col_group = player.position.1 / 50;

    (row_group * 50, col_group * 50)
}

fn get_target(player: &Player) -> (Position, (i32, i32)) {
    let face = get_face(player.position);
    let direction = Direction::from_vector(player.direction);

    let right = (0, 1);
    let left = (0, -1);
    let down = (1, 0);
    let up = (-1, 0);

    let face_1 = (0, 50);
    let face_2 = (0, 100);
    let face_3 = (50, 50);
    let face_4 = (100, 0);
    let face_5 = (100, 50);
    let face_6 = (150, 0);

    // We're going off the edge of the net
    // need to find _where_ it should go, then also permute its direction and position
    match (face, direction) {
        (Face::One, Direction::Left) => (face_4, right),
        (Face::One, Direction::Up) => (face_6, right),
        (Face::Two, Direction::Up) => (face_6, up),
        (Face::Two, Direction::Right) => (face_5, left),
        (Face::Two, Direction::Down) => (face_3, left),
        (Face::Three, Direction::Left) => (face_4, down),
        (Face::Three, Direction::Right) => (face_2, up),
        (Face::Four, Direction::Up) => (face_3, right),
        (Face::Four, Direction::Left) => (face_1, right),
        (Face::Five, Direction::Right) => (face_2, left),
        (Face::Five, Direction::Down) => (face_6, left),
        (Face::Six, Direction::Left) => (face_1, down),
        (Face::Six, Direction::Down) => (face_2, down),
        (Face::Six, Direction::Right) => (face_5, up),
        _ => {
            // dbg!(&direction, &face, &new_pos);
            panic!("Unexpected combination!")
        }
    }
}

fn warp_vector(
    player: &Player,
    from_top_left: Position,
    target_direction: (i32, i32),
    target_top_left: Position,
) -> Position {
    let half_side = 24.5;

    // centre to the origin, based on the original face
    // this lets us rotate that face so we can match its target
    let centered = (
        (player.position.0 - from_top_left.0) as f64 - half_side,
        (player.position.1 - from_top_left.1) as f64 - half_side,
    );

    // now reflect the position and original direction around y=-x
    let mut reflected = reflect_around_y_minus_x(centered);
    let mut original_direction =
        reflect_around_y_minus_x((player.direction.0 as f64, player.direction.1 as f64));

    // we want to rotate the original until it matches the reverse of the target
    let target_direction = (-target_direction.0 as f64, -target_direction.1 as f64);

    // rotate our original direction until it matches the target direction
    while original_direction != target_direction {
        original_direction = rotate_90(original_direction);
        reflected = rotate_90(reflected);
    }

    // translate our original with respect to its _new_ face
    let translated = (
        (reflected.0 + target_top_left.0 as f64) + 24.5,
        (reflected.1 + target_top_left.1 as f64) + 24.5,
    );

    (translated.0.round() as i32, translated.1.round() as i32)
}

fn rotate_90(vector: (f64, f64)) -> (f64, f64) {
    (vector.1, -vector.0)
}

fn reflect_around_y_minus_x(vector: (f64, f64)) -> (f64, f64) {
    (-vector.1, -vector.0)
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
                print!("{step}");
            } else if let Some(block) = map.blocks.get(&point) {
                print!(
                    "{}",
                    match block {
                        Block::Empty => '.',
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
    thread::sleep(time::Duration::from_millis(150));
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
            panic!(
                "Unexpected face! ({}, {}) in group ({}, {})",
                pos.0,
                pos.1,
                row_group + 1,
                col_group + 1
            )
        }
    }
}

// TODO add test cases to help test

#[test]
fn test_warp_correctly_translates_vectors() {
    for (player, expected) in _warp_test_cases() {
        let from_top_left = get_face_top_left(&player);
        let (target_face, target_direction) = get_target(&player);
        let point = warp_vector(&player, from_top_left, target_direction, target_face);

        assert_eq!(point, expected);
    }
}

fn _warp_test_cases() -> Vec<(Player, (i32, i32))> {
    vec![
        (
            Player {
                position: (0, 50),
                direction: (-1, 0),
            },
            (150, 0),
        ),
        (
            Player {
                position: (0, 50),
                direction: (0, -1),
            },
            (149, 0),
        ),
        (
            Player {
                position: (0, 107),
                direction: (-1, 0),
            },
            (199, 7),
        ),
        (
            Player {
                position: (0, 60),
                direction: (-1, 0),
            },
            (160, 0),
        ),
        (
            Player {
                position: (57, 99),
                direction: (0, 1),
            },
            (49, 107),
        ),
        (
            Player {
                position: (100, 99),
                direction: (0, 1),
            },
            (49, 149),
        ),
    ]
}
