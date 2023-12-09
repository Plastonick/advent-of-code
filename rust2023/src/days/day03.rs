use crate::common::get_lines;
use crate::Args;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Number {
    value: isize,
    row: isize,
    columns: Vec<isize>,
}

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day03-test")
    } else {
        get_lines("day03")
    };

    let symbol_locations = get_symbol_locations(&lines);
    let numbers = get_numbers(&lines);

    // dbg!(&numbers);

    let part_number_sum: isize = numbers
        .iter()
        .filter(|&x| is_included(x, &symbol_locations))
        .map(|x| x.value)
        .sum();

    if !args.no_answers {
        println!("Day 3, Part 1: The part number sum is {part_number_sum}");
        println!("Day 3, Part 2: TODO");
    }

    ("".to_string(), "".to_string())
}

fn is_included(number: &Number, symbols: &HashSet<(isize, isize)>) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            for col in &number.columns {
                if symbols.contains(&(number.row + i, col + j)) {
                    return true;
                }
            }
        }
    }

    false
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
                columns: digits.range().map(|x| x as isize).collect::<Vec<isize>>(),
            })
        })
        .collect::<Vec<_>>()
}
