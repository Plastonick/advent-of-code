use clap::{Parser, command};

mod days;
mod common;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Which day to run
   #[arg(short, long)]
   day: u16,

   /// Which part to run
   #[arg(short, long, default_value_t = 1)]
   part: u8,
}

fn main() {
    let args = Args::parse();

    common::get_lines("day04");

    match args.day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        4 => days::day04::run(),
        _ => println!("I haven't done this day yet ;(")
    };
}
