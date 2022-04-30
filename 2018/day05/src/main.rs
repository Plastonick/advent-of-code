use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong!");
    let polymer = contents.as_bytes().to_vec();

    println!("original length: {}", polymer.len());

    let reacted = react_polymer(&polymer);

    println!("part 1: length {}", reacted.len());

    let start_type = 'a' as u8;
    let end_type = 'z' as u8;
    let mut best = reacted.len();

    for unit_type in start_type..end_type + 1 {
        let polymer_without = polymer_without(&polymer, unit_type);
        let reacted_without = react_polymer(&polymer_without);

        best = if best > reacted_without.len() {
            reacted_without.len()
        } else {
            best
        }
    }

    println!("part 2: length {}", best);
}

fn polymer_without(original: &Vec<u8>, unit: u8) -> Vec<u8> {
    let mut polymer = original.clone();
    let mut i = 0;

    while i < polymer.len() {
        let unit_type = polymer[i];

        if unit_type == unit || polymer[i] == unit - 32 {
            polymer.remove(i);
        } else {
            i += 1;
        }
    }

    polymer
}

fn react_polymer(original: &Vec<u8>) -> Vec<u8> {
    let mut polymer = original.clone();
    let mut i = 0;

    while i < polymer.len() - 1 {
        let char1 = polymer[i];
        let char2 = polymer[i + 1];

        let polarised = if char1 > char2 {
            char1 - char2 == 32
        } else {
            char2 - char1 == 32
        };

        if polarised {
            polymer.remove(i);
            polymer.remove(i);

            i = if i >= 1 { i - 1 } else { 0 };
        } else {
            i += 1
        }
    }

    polymer
}
