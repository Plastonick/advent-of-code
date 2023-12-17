use crate::common::{get_file_contents, get_lines, rotate_270, rotate_90, Answer};
use crate::{get_epoch_ms, Args};
use std::collections::HashMap;

pub fn run(_args: &Args) -> Answer {
    let contents = if _args.test {
        get_file_contents("day14-test")
    } else {
        get_file_contents("day14")
    };

    let map = build_map(&contents);

    let tilted_north = rotate_90(tilt_map_west(rotate_270(map.clone())));
    let total_load_after_north_tilt = load_on_north_beam(&tilted_north);
    let load_after_1_000_000_000_cycles = load_on_north_beam(&cycle_n(map, 1_000_000_000));

    (
        total_load_after_north_tilt.to_string(),
        load_after_1_000_000_000_cycles.to_string(),
    )
}

fn build_map(contents: &String) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn map_signature(map: &Vec<Vec<char>>) -> String {
    map.iter()
        .map(|r| r.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn print_map(map: &Vec<Vec<char>>) {
    println!("{}\n\n", map_signature(&map))
}

fn load_on_north_beam(map: &Vec<Vec<char>>) -> usize {
    let map_height = map.len();

    map.iter()
        .enumerate()
        .map(|(r, row)| (map_height - r) * row.iter().filter(|&&x| x == 'O').count())
        .sum::<usize>()
}

fn cycle_n(map: Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut map = map;
    let mut visited: HashMap<String, usize> = HashMap::new();
    let mut indexed: HashMap<usize, String> = HashMap::new();
    let mut maybe_period = None;

    for i in 1..=n {
        map = cycle(map);

        let key = map_signature(&map);

        if let Some(&index) = visited.get(&key) {
            // we've been here before! We have a repeat!
            // store the period start and end (end is one less, since we're currently at the start of next period)
            maybe_period = Some((index, i));

            break;
        }

        indexed.insert(i, key.clone());
        visited.insert(key, i + 1);
    }

    if let Some((start, end)) = maybe_period {
        let period_length = end - start + 1;
        let target_mod = n % period_length;

        // need to find number x where start <= x < end, such that x % period_length = target_mod
        // there's definitely a better way! 😅
        let target = (start..end)
            .into_iter()
            .filter(|&x| x % period_length == target_mod)
            .collect::<Vec<usize>>()[0];

        if let Some(map_content) = indexed.get(&target) {
            return build_map(map_content);
        }
    }

    map
}

fn cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // rotate north, west, south, east
    let mut cycled_map = rotate_270(map);

    for _ in 0..4 {
        cycled_map = rotate_90(tilt_map_west(cycled_map));
    }

    rotate_90(cycled_map)
}

fn tilt_map_west(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    map.iter().map(tilt_row).collect()
}

fn tilt_row(row: &Vec<char>) -> Vec<char> {
    let mut tilted = row.clone();
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        for i in 1..tilted.len() {
            let a_index = tilted.len() - i;
            let b_index = a_index - 1;

            let a = tilted[a_index];
            let b = tilted[b_index];

            if b == '.' && a == 'O' {
                // we can swap a and b
                (tilted[a_index], tilted[b_index]) = (tilted[b_index], tilted[a_index]);
                has_changed = true;
            }
        }
    }

    tilted
}
