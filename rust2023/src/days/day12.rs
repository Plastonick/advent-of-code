use crate::common::{get_lines, Answer};
use crate::Args;
use cached::proc_macro::cached;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

impl Spring {
    fn as_char(self) -> char {
        match self {
            Spring::Operational => '.',
            Spring::Broken => '#',
            Spring::Unknown => '?',
        }
    }
}

pub fn run(args: &Args) -> Answer {
    let lines = if args.test {
        get_lines("day12-test")
    } else {
        get_lines("day12")
    };

    let patterns = lines
        .iter()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(pattern, numbers)| {
            (
                remove_dot_runs(pattern)
                    .chars()
                    .map(|ch| match ch {
                        '.' => Spring::Operational,
                        '#' => Spring::Broken,
                        '?' => Spring::Unknown,
                        _ => panic!("Unexpected spring character"),
                    })
                    .collect::<Vec<_>>(),
                numbers
                    .split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let part1 = patterns
        .iter()
        .map(|(pattern, ctgs_counts)| dp_matches(&pattern, &ctgs_counts, 0))
        .sum::<u64>();

    let part2 = patterns
        .iter()
        .map(|(pattern, ctgs_counts)| {
            let mut unfolded_pattern = pattern.clone();
            for _ in 0..4 {
                unfolded_pattern.push(Spring::Unknown);
                unfolded_pattern.extend(pattern.clone());
            }

            (unfolded_pattern, ctgs_counts.repeat(5))
        })
        .map(|(pattern, ctgs_counts)| dp_matches(&pattern, &ctgs_counts, 0))
        .sum::<u64>();

    // part 2
    // 290084770204 is too low

    (part1.to_string(), part2.to_string())
}

#[cached(
    key = "String",
    convert = r#"{ dp_key2(pattern, exp_counts, curr_ctgs) }"#
)]
fn dp_matches(pattern: &[Spring], exp_counts: &[u8], curr_ctgs: u8) -> u64 {
    // this is the easiest one to get right! We have no expected broken left, so it's either 1 or 0
    // this also makes future calculations much easier...
    if exp_counts.is_empty() {
        return if pattern.iter().any(|x| x == &Spring::Broken) {
            0
        } else {
            1
        };
    }

    if pattern.is_empty() {
        // we've already established exp_counts is non-empty, so we need our current run of ctgs broken to be the
        // expected, and for there to be no more expected
        return if curr_ctgs == exp_counts[0] && exp_counts.len() == 1 {
            1
        } else {
            0
        };
    }

    let next_pattern = &pattern[1..];

    // if we're currently on a contiguous run of broken springs, the next spring type is fixed
    if curr_ctgs > 0 {
        let needs_broken = curr_ctgs < exp_counts[0];
        if needs_broken {
            // we're currently on a run of broken springs, and we're not there yet; so needs to be broken
            if pattern[0] == Spring::Operational {
                0
            } else {
                dp_matches(next_pattern, exp_counts, curr_ctgs + 1)
            }
        } else {
            // we're on a run of broken ones, and we've hit the limit; so next one needs to be operational
            if pattern[0] == Spring::Broken {
                0
            } else {
                dp_matches(next_pattern, &exp_counts[1..], 0)
            }
        }
    } else {
        match pattern[0] {
            Spring::Operational => dp_matches(next_pattern, exp_counts, 0),
            Spring::Broken => dp_matches(next_pattern, exp_counts, 1),
            Spring::Unknown => {
                let as_operational = dp_matches(next_pattern, exp_counts, 0);
                let as_broken = dp_matches(next_pattern, exp_counts, 1);

                as_operational + as_broken
            }
        }
    }
}

fn remove_dot_runs(pattern: &str) -> String {
    let mut output_pattern = String::with_capacity(pattern.len());
    let mut prev_is_dot = false;
    for char in pattern.chars() {
        if char == '.' {
            // if the previous character was a dot, and so was this one; also not interesting
            if prev_is_dot == true {
                continue;
            }
        }

        output_pattern.push(char);
        prev_is_dot = char == '.';
    }

    output_pattern
}

fn dp_key2(pattern: &[Spring], ctgs_counts: &[u8], curr_ctgs: u8) -> String {
    let mut out = String::with_capacity(1 + pattern.len() + ctgs_counts.len() * 2);

    for spring in pattern {
        out.push(spring.as_char());
    }

    for count in ctgs_counts {
        out.push('-');
        out.push(*count as char);
    }

    out.push(curr_ctgs as char);

    out
}
