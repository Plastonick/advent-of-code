use crate::common::get_lines;
use crate::Args;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
struct Number {
    value: isize,
    row: isize,
    column_start: isize,
    column_end: isize,
}

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day03-test")
    } else {
        get_lines("day03")
    };

    let symbol_locations = get_symbol_locations(&lines);
    let star_locations = get_star_locations(&lines);
    let numbers = get_numbers(&lines);

    let part_number_sum: isize = numbers
        .iter()
        .filter(|&x| adjacent_symbol(x, &symbol_locations).is_some())
        .map(|x| x.value)
        .sum();

    let gear_ratio_sum = numbers
        .iter()
        .map(|x| (x, adjacent_symbol(x, &star_locations)))
        .filter_map(|(&x, star_opt)| match star_opt {
            Some(star) => Some((x, star)),
            None => None,
        })
        .fold(HashMap::new(), |mut acc, (x, star)| {
            acc.entry(star).or_insert_with(Vec::new).push(x);
            acc
        })
        .iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() == 2 {
                Some(numbers.iter().map(|x| x.value).product::<isize>())
            } else {
                None
            }
        })
        .sum::<isize>();

    (part_number_sum.to_string(), gear_ratio_sum.to_string())
}

fn adjacent_symbol(number: &Number, symbols: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    for i in -1..=1 {
        for j in -1..=1 {
            for col in number.column_start..number.column_end {
                if symbols.contains(&(number.row + i, col + j)) {
                    return Some((number.row + i, col + j));
                }
            }
        }
    }

    None
}

fn get_symbol_locations(lines: &Vec<String>) -> HashSet<(isize, isize)> {
    lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| is_special_char(*char))
                .map(move |(col, _)| (row as isize, col as isize))
        })
        .flatten()
        .collect::<HashSet<_>>()
}

fn get_star_locations(lines: &Vec<String>) -> HashSet<(isize, isize)> {
    lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, char)| *char == '*')
                .map(move |(col, _)| (row as isize, col as isize))
        })
        .flatten()
        .collect::<HashSet<_>>()
}

fn is_special_char(c: char) -> bool {
    match c {
        '0'..='9' => false,
        '.' => false,
        _ => true,
    }
}

fn get_numbers(lines: &Vec<String>) -> Vec<Number> {
    lines
        .iter()
        .enumerate()
        .map(|(row, line)| get_numbers_from_line(row, line))
        .flatten()
        .collect()
}

fn get_numbers_from_line(row: usize, line: &String) -> Vec<Number> {
    let regex = Regex::new(r"(\d+)").unwrap();

    regex
        .find_iter(line)
        .filter_map(|digits| {
            Some(Number {
                value: digits.as_str().parse::<isize>().unwrap(),
                row: row as isize,
                column_start: digits.start() as isize,
                column_end: digits.end() as isize,
            })
        })
        .collect::<Vec<_>>()
}
