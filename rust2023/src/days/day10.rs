use crate::common::get_lines;
use crate::Args;

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day10-test")
    } else {
        get_lines("day10")
    };

    if !args.no_answers {
        println!("Day 10, Part 1: TODO");
        println!("Day 10, Part 2: TODO");
    }

    ("".to_string(), "".to_string())
}
