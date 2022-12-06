use std::collections::HashSet;

use crate::common::get_file_contents;

pub fn run() {
    let buffer = get_file_contents("day06");

    let chars = buffer.chars();
    let mut last_four: Vec<char> = Vec::new();
    let mut counter = 0;

    for character in chars {
        last_four.push(character);
        counter += 1;

        if last_four.len() > 4 {
            last_four.remove(0);
        } else if last_four.len() < 4 {
            continue;
        }

        if are_unique(&last_four) {
            break;
        }
    }

    println!(
        "Day 06, Part 1: The first all unique group of characters occurs at position {}",
        counter
    );
}

fn are_unique(characters: &Vec<char>) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(characters.into_iter());

    set.len() == characters.len()
}
