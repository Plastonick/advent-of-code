use std::{collections::HashMap, fs};

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(contents: &String) -> i32 {
    let mut exactly_twice = 0;
    let mut exactly_thrice = 0;

    let lines = contents.lines();

    for line in lines.into_iter() {
        let mut char_counts: HashMap<u8, i32> = HashMap::new();
        for byte in line.as_bytes().into_iter() {
            if !char_counts.contains_key(byte) {
                char_counts.insert(*byte, 1);
            } else {
                char_counts.insert(*byte, char_counts.get(byte).unwrap() + 1);
            }
        }

        let mut has_twice = false;
        let mut has_thrice = false;

        for counts in char_counts.into_values() {
            if !has_twice && counts == 2 {
                exactly_twice += 1;
                has_twice = true;
            }

            if !has_thrice && counts == 3 {
                exactly_thrice += 1;
                has_thrice = true;
            }
        }
    }

    return exactly_twice * exactly_thrice;
}

fn part2(contents: &String) -> String {
    for line in contents.lines().into_iter() {
        for comparison in contents.lines().into_iter() {
            let similar = similarity(line, comparison);
            if similar.len() + 1 == line.len() {
                return similar;
            }
        }
    }

    return "Not found".to_string();
}

fn similarity(a: &str, b: &str) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut similarity = "".to_string();

    for i in 0..a_bytes.len() {
        if a_bytes[i] == b_bytes[i] {
            similarity.push(a_bytes[i] as char);
        }
    }

    similarity
}
