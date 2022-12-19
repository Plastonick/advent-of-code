use std::cmp::Ordering;

use crate::{common::get_file_contents, Args};

pub fn run(args: &Args) {
    let file = if args.test {
        get_file_contents("day13-test")
    } else {
        get_file_contents("day13")
    };

    let pairs: Vec<_> = file
        .split("\n\n")
        .map(|x| x.trim().split_once('\n').unwrap())
        .collect();

    let mut part_1_sum = 0;
    let mut all_lines = vec!["[[2]]", "[[6]]"];

    for (i, pair) in pairs.iter().enumerate() {
        let result = compare(pair.0, pair.1);

        part_1_sum += match result {
            Some(false) => 0,
            Some(true) => i + 1,
            None => {
                println!("Uh oh, this shouldn't happen");
                0
            }
        };

        all_lines.push(pair.0);
        all_lines.push(pair.1);
    }

    all_lines.sort_by(|x, y| match compare(x, y) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let div2_index = all_lines.iter().position(|&x| x == "[[2]]").unwrap() + 1;
    let div6_index = all_lines.iter().position(|&x| x == "[[6]]").unwrap() + 1;

    if !args.no_answers {
        println!(
            "Day 13, Part 1: The sum of the indices of the ordered pairs is {}",
            part_1_sum
        );
        println!(
            "Day 13, Part 2: The decoder key is {}",
            div2_index * div6_index
        );
    }
}

fn compare(left: &str, right: &str) -> Option<bool> {
    let left_elements = get_elements(&left);
    let right_elements = get_elements(&right);

    for (i, left_el) in left_elements.iter().enumerate() {
        if i >= right_elements.len() {
            return Some(false);
        }

        let right_el = &right_elements[i];

        let left_is_list = left_el.find('[').unwrap_or(1) == 0;
        let right_is_list = right_el.find('[').unwrap_or(1) == 0;

        // if neither are lists, compare the numeric values
        if !left_is_list && !right_is_list {
            // compare directly!

            let left_num = left_el.parse::<u8>().unwrap();
            let right_num = right_el.parse::<u8>().unwrap();

            if left_num < right_num {
                return Some(true);
            }

            if right_num < left_num {
                return Some(false);
            }

            // no valid comparison to be made, try the next element
            continue;
        }

        // if one is a list, and one an int... find the int, make it a list, then compare as lists!
        let list_left_el = if !left_is_list && right_is_list {
            let formatted = format!("{}{}{}", "[", left_el, "]");

            formatted
        } else {
            left_el.to_owned()
        };

        let list_right_el = if left_is_list && !right_is_list {
            let formatted = format!("{}{}{}", "[", right_el, "]");

            formatted
        } else {
            right_el.to_owned()
        };

        // now both are lists... compare them as lists!
        let sub_result = compare(&list_left_el, &list_right_el);

        if sub_result.is_some() {
            return sub_result;
        }

        // no real result, continue
    }

    // all of the elements of the left are matched to the right
    // are there fewer left elements? If so, it's the correct order!
    if left.len() < right.len() {
        return Some(true);
    }

    // both lists are identically ordered
    None
}

fn get_elements(list: &str) -> Vec<String> {
    let mut index = 1;
    let mut output: Vec<String> = Vec::new();

    while index < list.len() {
        let (el, size) = get_next_element(&list[index..list.len()]);

        if el == "" {
            break;
        }

        index += size;
        output.push(el);
    }

    output
}

fn get_next_element(el: &str) -> (String, usize) {
    let mut output = String::new();
    let mut depth = 0;

    for (index, character) in el.chars().enumerate() {
        match character {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ => (),
        }

        // we're not indented, and we've hit a comma, we've completed our el, move on
        if depth == 0 && character == ',' {
            return (output, index + 1);
        }

        // we've gone a level too deep! We've probably finished the element, return now
        if depth == -1 {
            return (output, index);
        }

        // add the character to our element
        output.push(character);
    }

    (output, el.len())
}
