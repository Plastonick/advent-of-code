use std::collections::HashSet;

use crate::common::get_file_contents;

pub fn run() {
    let buffer = get_file_contents("day06");

    let chars = buffer.chars();
    let mut last_four: Vec<char> = Vec::new();
    let mut last_fourteen: Vec<char> = Vec::new();
    let mut counter = 0;
    let mut start_of_packet = 0;
    let mut start_of_message = 0;

    for character in chars {
        last_four.push(character);
        last_fourteen.push(character);
        counter += 1;

        if last_four.len() > 4 {
            last_four.remove(0);
        }

        if last_fourteen.len() > 14 {
            last_fourteen.remove(0);
        }

        if last_four.len() == 4 && are_unique(&last_four) && start_of_packet == 0 {
            start_of_packet = counter;
        }

        if last_fourteen.len() == 14 && are_unique(&last_fourteen) && start_of_message == 0 {
            start_of_message = counter;
        }
    }

    println!(
        "Day 06, Part 1: The start of packet marker occurs at position {}",
        start_of_packet
    );
    println!(
        "Day 06, Part 2: The start of message marker occurs at position {}",
        start_of_message
    );
}

fn are_unique(characters: &Vec<char>) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(characters.into_iter());

    set.len() == characters.len()
}
