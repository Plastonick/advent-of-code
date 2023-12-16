use ascii_table::{Align, AsciiTable};
use clap::{command, Parser};
use std::time::{SystemTime, UNIX_EPOCH};

mod common;
mod days;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Which day to run
    #[arg(short, long, default_value_t = 1)]
    day: u16,

    /// Whether to run all days
    #[arg(short, long, default_value_t = false)]
    all: bool,

    /// Whether to display visual output
    #[arg(long, default_value_t = false)]
    visual: bool,

    /// Whether to display visual output
    #[arg(long, default_value_t = false)]
    no_answers: bool,

    /// Whether to display visual output
    #[arg(long, default_value_t = false)]
    test: bool,

    /// Whether to display visual output
    #[arg(long, default_value_t = false)]
    time: bool,
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
    ];

    if args.all {
        let mut duration_data = Vec::new();
        let all_start = get_epoch_ms();

        for (day, func) in days.iter().enumerate() {
            let func_start = get_epoch_ms();
            let (part1, part2) = func(&args);
            let func_duration = get_epoch_ms() - func_start;

            if args.no_answers {
                duration_data.push(vec![format!("{}", day + 1), format!("{func_duration:.3}s")]);
            } else {
                duration_data.push(vec![
                    format!("{}", day + 1),
                    format!("{func_duration:.3}s"),
                    part1,
                    part2,
                ]);
            }
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
        if !args.no_answers {
            ascii_table
                .column(2)
                .set_header("Part 1")
                .set_align(Align::Right);
            ascii_table
                .column(3)
                .set_header("Part 2")
                .set_align(Align::Right);
        }

        ascii_table.print(duration_data);
    } else {
        let day_opt = days
            .iter()
            .enumerate()
            .position(|(x, _)| (x + 1) as u16 == args.day);

        if let Some(day_index) = day_opt {
            let day = day_index + 1;
            let func_start = get_epoch_ms();
            let (part1, part2) = days[day_index](&args);
            let func_duration = get_epoch_ms() - func_start;

            if args.time {
                println!("Calculated in {func_duration:.3}s");
            }

            if !args.no_answers {
                println!("Day {day}, Part 1: {part1}");
                println!("Day {day}, Part 2: {part2}");
            }
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
