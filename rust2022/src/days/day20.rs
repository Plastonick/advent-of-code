use std::collections::VecDeque;

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
        .enumerate()
        .collect::<VecDeque<_>>();

    let part1 = calc_grove_number(&numbers, 1, 1);
    let part2 = calc_grove_number(&numbers, 811589153, 10);

    if !args.no_answers {
        println!("Day 20, Part 1: The Grove Number is {}", part1);
        println!("Day 20, Part 2: The Larger Grove Number is {}", part2);
    }

    (part1.to_string(), part2.to_string())
}

fn calc_grove_number(
    input: &VecDeque<(usize, isize)>,
    decryption_key: isize,
    shuffles: u8,
) -> isize {
    let mut numbers = input.clone();

    for _ in 0..shuffles {
        for num_index in 0..numbers.len() {
            let index = numbers
                .iter()
                .enumerate()
                .find_map(|(actual, (orig, _))| (*orig == num_index).then_some(actual))
                .unwrap();

            numbers.rotate_left(index);
            let element = numbers.pop_front().unwrap();
            let moves = (element.1 * decryption_key).rem_euclid(numbers.len() as isize) as usize;

            numbers.rotate_left(moves);
            numbers.push_front(element);
        }
    }

    let zero_index = numbers
        .iter()
        .enumerate()
        .find_map(|(i, (_, val))| (*val == 0).then_some(i))
        .unwrap();

    let grove_number = numbers[(zero_index + 1000) % numbers.len()].1
        + numbers[(zero_index + 2000) % numbers.len()].1
        + numbers[(zero_index + 3000) % numbers.len()].1;

    grove_number * decryption_key
}
