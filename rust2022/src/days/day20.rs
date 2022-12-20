use std::collections::HashMap;

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day20-test")
    } else {
        get_lines("day20")
    };

    let numbers = lines
        .iter()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let part1 = calc_grove_number(&numbers, 1, 1);
    let part2 = calc_grove_number(&numbers, 811589153, 10);

    if !args.no_answers {
        println!("Day 20, Part 1: The Grove Number is {}", part1);
        println!("Day 20, Part 2: The Larger Grove Number is {}", part2);
    }

    (part1.to_string(), part2.to_string())
}

fn iterate_sequence(
    index: usize,
    sequence: &mut HashMap<usize, (usize, isize, usize)>,
    decryption_key: isize,
) {
    // get the neighbouring elements
    let (prev, number, mut next) = sequence[&index];

    // close the sequence
    sequence.insert(prev, (sequence[&prev].0, sequence[&prev].1, next));
    sequence.insert(next, (prev, sequence[&next].1, sequence[&next].2));

    // remember to ignore the original element
    let sequence_length = (sequence.len() as isize) - 1;

    // find where we need to move to
    let encrypt_number = number * decryption_key;
    let moves: isize = (encrypt_number
        + (sequence_length * ((encrypt_number.abs() / sequence_length) + 1)))
        % sequence_length;

    for _ in 0..moves {
        (_, _, next) = sequence[&next];
    }

    let new_prev = sequence[&next].0;

    // inject our number into the sequence at this point
    sequence.insert(index, (new_prev, number, next));

    // revalue the previous and next number
    sequence.insert(
        new_prev,
        (sequence[&new_prev].0, sequence[&new_prev].1, index),
    );
    sequence.insert(next, (index, sequence[&next].1, sequence[&next].2));
}

fn calc_grove_number(numbers: &Vec<isize>, decryption_key: isize, shuffles: u8) -> isize {
    let mut zero_index = 0;
    let mut sequence = HashMap::with_capacity(numbers.len());

    for (i, number) in numbers.iter().enumerate() {
        let prev_index = (numbers.len() + i - 1) % numbers.len();
        let next_index = (i + 1) % numbers.len();

        if number == &0 {
            zero_index = i;
        }

        sequence.insert(i, (prev_index, number.to_owned(), next_index));
    }
    for _ in 0..shuffles {
        for (index, _) in numbers.iter().enumerate() {
            iterate_sequence(index, &mut sequence, decryption_key);
        }
    }

    let mut grove_number = 0;
    let (_, mut number, mut next) = sequence[&zero_index];

    for i in 0..=3000 {
        if i % 1000 == 0 {
            grove_number += number;
        }
        (_, number, next) = sequence[&next];
    }

    grove_number * decryption_key
}

fn _print(sequence: &HashMap<usize, (usize, isize, usize)>) {
    let first_index = if sequence.contains_key(&0) { 0 } else { 1 };

    let (_, mut number, mut next) = sequence[&first_index];

    for _ in 0..sequence.len() {
        print!("{}, ", number);
        // println!("{} -> {} -> {}", prev, el, next);
        (_, number, next) = sequence[&next];
    }

    println!();
}
