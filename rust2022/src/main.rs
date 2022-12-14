use ascii_table::{Align, AsciiTable};
use clap::{command, Parser};
use std::time::{SystemTime, UNIX_EPOCH};

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

    let days = [
        days::day01::run,
        days::day02::run,
        days::day03::run,
        days::day04::run,
        days::day05::run,
        days::day06::run,
        days::day07::run,
        days::day08::run,
        days::day09::run,
        days::day10::run,
        days::day11::run,
        days::day12::run,
        days::day13::run,
        days::day14::run,
    ];

    if args.all {
        let mut duration_data = Vec::new();
        let all_start = get_epoch_ms();

        for (day, func) in days.iter().enumerate() {
            let func_start = get_epoch_ms();
            func(args.visual);
            let func_duration = get_epoch_ms() - func_start;

            duration_data.push(vec![format!("{}", day + 1), format!("{func_duration:.3}s")]);
        }

        let all_duration = get_epoch_ms() - all_start;

        duration_data.push(vec![String::from("Total"), format!("{all_duration:.3}s")]);

        let mut ascii_table = AsciiTable::default();
        ascii_table
            .column(0)
            .set_header("Day")
            .set_align(Align::Right);
        ascii_table
            .column(1)
            .set_header("Duration")
            .set_align(Align::Right);

        ascii_table.print(duration_data);
    } else {
        let day = days
            .iter()
            .enumerate()
            .position(|(x, _)| (x + 1) as u16 == args.day);

        if day.is_some() {
            days[day.unwrap()](args.visual);
        } else {
            println!("I haven't done this day yet ;(");
        }
    }
}

fn get_epoch_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
