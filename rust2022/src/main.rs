use clap::{Parser, command};

mod days;
mod common;

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
}

fn main() {
    let args = Args::parse();

    if args.all {
        days::day01::run();
        days::day02::run();
        days::day03::run();
        days::day04::run();
        days::day05::run();
    } else {
        match args.day {
            1 => days::day01::run(),
            2 => days::day02::run(),
            3 => days::day03::run(),
            4 => days::day04::run(),
            5 => days::day05::run(),
            _ => println!("I haven't done this day yet ;(")
        };
    }
}
