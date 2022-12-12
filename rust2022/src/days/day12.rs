use std::collections::{HashMap, HashSet};

use crate::common::{get_file_contents, get_lines};

pub fn run() {
    let file = get_file_contents("day12");
    let width = file.find('\n').unwrap();

    let start_pos = unwrap_linear_position(file.find('S').unwrap(), width);
    let end_pos = unwrap_linear_position(file.find('E').unwrap(), width);

    let lines = get_lines("day12");
    let mut elevation_map: Vec<Vec<u8>> = lines.iter().map(|x| x.as_bytes().to_owned()).collect();

    let mut a_points = vec![];

    // todo neaten this up into a .map
    for row in 0..elevation_map.len() {
        let map_row = &elevation_map[row];
        for el in 0..map_row.len() {
            let height = height_of_byte(elevation_map[row][el]);
            elevation_map[row][el] = height;

            if height == height_of_byte('a' as u8) {
                a_points.push((row, el));
            }
        }
    }

    let mut sources: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    sources.insert(0, HashSet::from_iter(vec![start_pos]));

    let part1_distance = solve_map(&mut sources, end_pos, &elevation_map);

    sources.clear();
    sources.insert(0, HashSet::from_iter(a_points));

    let part2_distance = solve_map(&mut sources, end_pos, &elevation_map);

    println!(
        "Day 12, Part 1: Min distance from 'S' point is {}",
        part1_distance
    );
    println!(
        "Day 12, Part 2: Min distance from any 'a' point is {}",
        part2_distance
    );
}

fn solve_map(
    sources: &mut HashMap<usize, HashSet<(usize, usize)>>,
    target: (usize, usize),
    elevation_map: &Vec<Vec<u8>>,
) -> usize {
    // a list of points for each distance

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut distance: usize = 0;
    let mut min_distance = 0;

    while min_distance == 0 {
        let targets_at_distance = sources.get(&distance).expect("How?");
        let mut new_neighbours: HashSet<(usize, usize)> = HashSet::new();

        distance += 1;
        for point in targets_at_distance {
            let point_height = elevation_map[point.0][point.1];

            let neighbours = get_surrounding_points(
                point,
                (elevation_map.len(), elevation_map.first().unwrap().len()),
            );

            for neighbour in neighbours {
                // we've already found a shorter route, let's ignore it and move on
                if visited.contains(&neighbour) {
                    continue;
                }

                let surrounding_point_height = elevation_map[neighbour.0][neighbour.1];

                // can we even visit this point?
                if surrounding_point_height as isize - point_height as isize > 1 {
                    // no we cannot... continue
                    continue;
                }

                // is this point the end!?
                if neighbour == target {
                    // it is! We're done!
                    min_distance = distance;
                }

                // We've not _already_ visited this and we _can_ visit this, let's visit it!
                visited.insert(neighbour);
                new_neighbours.insert(neighbour);
            }
        }

        if new_neighbours.len() == 0 {
            println!("Breaking early, no new neighbours found");
            break;
        }
        sources.insert(distance, new_neighbours);
    }

    min_distance
}

fn unwrap_linear_position(pos: usize, width: usize) -> (usize, usize) {
    // since the linear position includes new lines, we need to artificially increase the width by 1
    ((pos / (width + 1)), pos % (width + 1))
}

fn get_surrounding_points(
    point: &(usize, usize),
    max_dimensions: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];

    for i in -1..=1 {
        let point_i = point.0 as isize - i;

        if point_i < 0 || point_i as usize >= max_dimensions.0 {
            continue;
        }

        for j in -1..=1 {
            if i * j != 0 || i == j {
                // we don't allow moving diagonally
                continue;
            }

            let point_j = point.1 as isize - j;

            if point_j < 0 || point_j as usize >= max_dimensions.1 {
                continue;
            }

            output.push((point_i as usize, point_j as usize));
        }
    }

    output
}

fn height_of_byte(byte: u8) -> u8 {
    match byte as char {
        'S' => 1,
        'E' => 26,
        _ => 1 + byte - 'a' as u8,
    }
}
