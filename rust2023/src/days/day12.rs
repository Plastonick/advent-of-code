use crate::common::{get_lines, Answer};
use crate::Args;

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
                pattern.chars().collect::<Vec<_>>(),
                numbers
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let count_sum = patterns
        .into_iter()
        .map(|(p, c)| brute_force_valid(p, &c))
        .sum::<usize>();

    (count_sum.to_string(), "".to_string())
}

fn brute_force_valid(pattern: Vec<char>, counts: &Vec<usize>) -> usize {
    let all_patterns = get_all(pattern);

    let valid_count = all_patterns
        .iter()
        .map(|p| {
            get_contiguous_counts(&p)
                .iter()
                .filter_map(|(ch, count)| if ch == &'#' { Some(*count) } else { None })
                .collect::<Vec<usize>>()
        })
        .filter(|x| x == counts)
        .count();

    valid_count
}

fn get_all(pattern: Vec<char>) -> Vec<Vec<char>> {
    let question_mark_match = pattern.iter().position(|c| c == &'?');

    if let Some(index) = question_mark_match {
        let mut as_hash = pattern.clone();
        as_hash[index] = '#';
        let mut as_dot = pattern;
        as_dot[index] = '.';

        let mut patterns = get_all(as_hash);
        patterns.extend_from_slice(&get_all(as_dot));

        patterns
    } else {
        // no more `?` in the pattern, just return it!
        vec![pattern]
    }
}

fn count_valid(pattern: &Vec<char>, counts: &Vec<usize>) -> usize {
    let potential = get_contiguous_counts(pattern);

    2
}

fn get_contiguous_counts(pattern: &Vec<char>) -> Vec<(char, usize)> {
    let mut lengths: Vec<(char, usize)> = vec![];
    let mut maybe_previous: Option<char> = None;
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
