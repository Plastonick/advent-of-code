use std::collections::HashSet;

use crate::common::get_file_contents;

pub fn run() {
    let buffer = get_file_contents("day06");

    let bytes = buffer.as_bytes();
    let mut start_of_packet = 0;
    let mut start_of_message = 0;

    for i in 0..bytes.len() - 13 {
        if start_of_packet == 0 && are_unique(&bytes[i..i + 4]) {
            start_of_packet = i + 4;
        }

        if start_of_message == 0 && are_unique(&bytes[i..i + 14]) {
            start_of_message = i + 14;
        }
    }

    println!(
        "Day 6, Part 1: The start of packet marker occurs at position {}",
        start_of_packet
    );
    println!(
        "Day 6, Part 2: The start of message marker occurs at position {}",
        start_of_message
    );
}

fn are_unique(characters: &[u8]) -> bool {
    let set: HashSet<&u8> = HashSet::from_iter(characters.into_iter());

    set.len() == characters.len()
}
