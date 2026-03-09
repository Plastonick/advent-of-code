use crate::common::{get_lines, Answer};
use crate::Args;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Spring {
    Operational,
    Broken,
    Unknown,
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
                pattern
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
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let count_sum = patterns
        .into_iter()
        .map(|(p, c)| brute_force_valid(p, &c))
        .sum::<u32>();

    (count_sum.to_string(), "".to_string())
}

fn brute_force_valid(pattern: Vec<Spring>, counts: &[u32]) -> u32 {
    let all_patterns = get_all(pattern);

    let valid_count = all_patterns
        .iter()
        .map(|p| {
            get_contiguous_counts(&p)
                .iter()
                .filter_map(|(ch, count)| {
                    if ch == &Spring::Broken {
                        Some(*count)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u32>>()
        })
        .filter(|x| x == counts)
        .count();

    valid_count as u32
}

fn get_all(pattern: Vec<Spring>) -> Vec<Vec<Spring>> {
    let question_mark_match = pattern.iter().position(|c| c == &Spring::Unknown);

    if let Some(index) = question_mark_match {
        let mut as_hash = pattern.clone();

        as_hash[index] = Spring::Broken;
        let mut as_dot = pattern;
        as_dot[index] = Spring::Operational;

        let mut patterns = get_all(as_hash);
        patterns.extend_from_slice(&get_all(as_dot));

        patterns
    } else {
        // no more `?` in the pattern, just return it!
        vec![pattern]
    }
}

fn get_contiguous_counts(pattern: &[Spring]) -> Vec<(Spring, u32)> {
    let mut lengths: Vec<(Spring, u32)> = vec![];
    let mut maybe_previous: Option<Spring> = None;
    let mut current_length = 0;

    for &current_ch in pattern {
        if let Some(previous_element) = maybe_previous {
            if previous_element != current_ch {
                lengths.push((previous_element, current_length));
                current_length = 0;
            }
        }

        maybe_previous = Some(current_ch);
        current_length += 1;
    }

    // make sure we add the last set on, too!
    lengths.push((maybe_previous.unwrap(), current_length));

    lengths
}
