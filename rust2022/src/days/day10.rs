use crate::common::get_lines;

pub fn run(visual: bool) {
    let lines = get_lines("day10");

    let mut register = 1;
    let mut cycle = 1;
    let mut sum_signal_strength = 0;
    let mut line_num: isize = 0;

    for line in lines {
        let (wait, add) = if line == "noop" {
            (1, 0)
        } else {
            let (_, value) = line.split_once(' ').unwrap();

            (2, value.parse::<isize>().unwrap())
        };

        for _ in 0..wait {
            if (cycle + 20) % 40 == 0 {
                sum_signal_strength += register * cycle;
            }

            if visual {
                if register - (cycle - (40 * line_num)) <= 0
                    && register - (cycle - (40 * line_num)) >= -2
                {
                    print!("█");
                } else {
                    print!("░");
                }

                if cycle % 40 == 0 {
                    println!();
                    line_num += 1;
                }
            }

            cycle += 1;
        }

        register += add;
    }

    println!(
        "Day 10, Part 1: The total signal strength is {}",
        sum_signal_strength
    );
    if !visual {
        println!("Day 10, Part 2: This answer requires the --visual flag");
    }
}
