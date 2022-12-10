use crate::common::get_lines;

pub fn run() {
    let lines = get_lines("day10");

    let mut register = 1;
    let mut cycle = 1;
    let mut sum_signal_strength = 0;

    for line in lines {
        let (wait, add) = if line == "noop" {
            (1, 0)
        } else {
            let (_, value) = line.split_once(' ').unwrap();

            let int_value = value.parse::<isize>().unwrap();

            (2, int_value)
        };

        for _ in 0..wait {
            if (cycle + 20) % 40 == 0 {
                println!("{}, {}, {}", cycle, register, register * cycle);

                sum_signal_strength += register * cycle;
            }

            cycle += 1;
        }

        register += add;
    }

    println!(
        "Day 10, Part 1: The total signal strength is {}",
        sum_signal_strength
    );
}
