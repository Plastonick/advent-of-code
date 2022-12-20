use std::collections::HashSet;

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    let lines = get_lines("day03");
    let mut line_share_sum = 0;
    let mut group_sum = 0;

    let mut previous_groups: [HashSet<u8>; 2] = [HashSet::new(), HashSet::new()];
    let mut group_counter = 0;

    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);

        let line_types: HashSet<u8> = HashSet::from_iter(line.as_bytes().iter().cloned());
        let left_types: HashSet<u8> = HashSet::from_iter(left.as_bytes().iter().cloned());
        let right_types: HashSet<u8> = HashSet::from_iter(right.as_bytes().iter().cloned());

        let intersection = left_types.intersection(&right_types);

        for byte in intersection {
            line_share_sum += byte_priority(*byte);
        }

        if group_counter < 2 {
            previous_groups[group_counter] = line_types.clone();

            group_counter += 1;
        } else {
            let shared_values = line_types
                .into_iter()
                .filter(|x| previous_groups[0].contains(x))
                .filter(|x| previous_groups[1].contains(x));

            for value in shared_values {
                group_sum += byte_priority(value);
            }

            group_counter = 0;
        }
    }

    if !args.no_answers {
        println!("Day 3, Part 1: Per-line shared items priority sum: {line_share_sum}");
        println!("Day 3, Part 2: Per-group of three shared items priority sum: {group_sum}");
    }

    ("".to_string(), "".to_string())
}

fn byte_priority(byte: u8) -> u32 {
    let ascii_a = 'a' as u8;

    let priority = if byte >= ascii_a {
        1 + byte - ascii_a
    } else {
        27 + byte - 'A' as u8
    };

    u32::from(priority)
}
