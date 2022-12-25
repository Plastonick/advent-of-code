use std::cmp::max;

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day25-test")
    } else {
        get_lines("day25")
    };

    let snafu_total = lines
        .iter()
        .map(|x| x.to_owned())
        .reduce(sum_snafu)
        .expect("No answer");

    if !args.no_answers {
        println!("Day 25, Part 1: The sum of the snafu numbers is {snafu_total}");
    }

    (snafu_total.to_string(), "".to_string())
}

fn sum_snafu(a: String, b: String) -> String {
    let max_order = max(a.len(), b.len());
    let a_rev = a.chars().rev().collect::<Vec<_>>();
    let b_rev = b.chars().rev().collect::<Vec<_>>();

    let mut output = String::new();
    let mut remainder = 0;

    for i in 0..=max_order {
        let a_val = snafu_char_to_val(*a_rev.get(i).unwrap_or(&'0'));
        let b_val = snafu_char_to_val(*b_rev.get(i).unwrap_or(&'0'));

        let mut value = remainder + a_val + b_val;

        (remainder, value) = if value > 2 {
            (1, value - 5)
        } else if value >= -2 {
            (0, value)
        } else {
            (-1, value + 5)
        };

        output = format!("{}{output}", val_to_snafu_char(value))
    }

    output.strip_prefix('0').unwrap_or(&output).to_string()
}

fn snafu_char_to_val(ch: char) -> i8 {
    match ch {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => {
            panic!("Unexpectec snafu '{ch}'")
        }
    }
}

fn val_to_snafu_char(val: i8) -> char {
    match val {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => {
            panic!("Unexpectec val '{val}'")
        }
    }
}

#[test]
fn test_sum() {
    for (a, b, expected) in _sum_samples() {
        assert_eq!(expected, sum_snafu(a, b));
    }
}

fn _sum_samples() -> Vec<(String, String, String)> {
    vec![
        ("2=".to_string(), "2=".to_string(), "1=1".to_string()),
        ("22".to_string(), "22".to_string(), "10-".to_string()),
        ("2".to_string(), "2".to_string(), "1-".to_string()),
        ("1".to_string(), "1".to_string(), "".to_string()),
    ]
}
