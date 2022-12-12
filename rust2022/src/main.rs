use clap::{command, Parser};

mod common;
mod days;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long, default_value_t = 1)]
    day: u16,

    /// Whether to run all days
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Whether to display visual output
    #[arg(short, long, default_value_t = false)]
    visual: bool,
}

fn main() {
    let args = Args::parse();

    if args.all {
        days::day01::run();
        days::day02::run();
        days::day03::run();
        days::day04::run();
        days::day05::run();
        days::day06::run();
        days::day07::run();
        days::day08::run();
        days::day09::run(args.visual);
        days::day10::run(args.visual);
        days::day11::run();
        days::day12::run();
    } else {
        match args.day {
            1 => days::day01::run(),
            2 => days::day02::run(),
            3 => days::day03::run(),
            4 => days::day04::run(),
            5 => days::day05::run(),
            6 => days::day06::run(),
            7 => days::day07::run(),
            8 => days::day08::run(),
            9 => days::day09::run(args.visual),
            10 => days::day10::run(args.visual),
            11 => days::day11::run(),
            12 => days::day12::run(),
            _ => println!("I haven't done this day yet ;("),
        };
    }
}
