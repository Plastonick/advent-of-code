use crate::common::{get_lines, Answer};
use crate::Args;

pub fn run(args: &Args) -> Answer {
    let lines = if args.test {
        get_lines("day12-test")
    } else {
        get_lines("day12")
    };

    let patterns = lines
        .iter()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(pattern, numbers)| {
            (
                pattern.chars().collect::<Vec<_>>(),
                numbers
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let count_sum = patterns
        .iter()
        .map(|(p, c)| count_valid(p, c))
        .sum::<usize>();

    ("".to_string(), "".to_string())
}

fn count_valid(pattern: &Vec<char>, counts: &Vec<usize>) -> usize {
    let potential = get_contiguous_counts(pattern);

    dbg!(&potential);

    let pot = &potential.iter().map(|x| x.to_string()).collect::<String>();
    let act = &counts.iter().map(|x| x.to_string()).collect::<String>();

    println!("{} => {}", pot, act);

    2
}

// fn is_valid(pattern: &Vec<char>, counts: &Vec<usize>) -> bool {
//     get_counts(&pattern)
//         .iter()
//         .enumerate()
//         .find(|(i, &count)| count != *counts.get(*i).unwrap())
//         .is_none()
// }

fn get_contiguous_counts(pattern: &Vec<char>) -> Vec<usize> {
    let mut lengths: Vec<usize> = vec![];
    let mut prev_el: Option<char> = None;
    let mut current_length = 0;

    for ch in pattern {
        if ch == &'.' {
            continue;
        }

        // if element == current_element:
        //     current_length += 1
        // else:
        // if current_element is not None:
        //     lengths.append(current_length)
        // current_element = element
        // current_length = 1

        if prev_el != Some(*ch) {
            lengths.push(current_length);
            current_length = 0;
        }

        prev_el = Some(*ch);
        current_length += 1;
    }

    dbg!(&pattern, &lengths);

    lengths

    // let enumerated_pattern: Vec<(usize, char)> = pattern
    //     .iter()
    //     .collect::<String>()
    //     .replace(".", "")
    //     .chars()
    //     .enumerate()
    //     .collect::<Vec<_>>();
    //
    // let foo = enumerated_pattern
    //     .iter()
    //     .fold(Vec::new(), |mut acc, (i, el)| {
    //         let count = if i == &0 {
    //             1
    //         } else if let Some(&prev) = enumerated_pattern.get(i - 1) {
    //             if el == &prev.1 {
    //                 prev.0 + 1
    //             } else {
    //                 1
    //             }
    //         } else {
    //             1
    //         };
    //
    //         acc.push((el, count));
    //
    //         acc
    //     })
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, (ch, count))| {
    //
    //     })
    //     .collect::<Vec<_>>();
}
