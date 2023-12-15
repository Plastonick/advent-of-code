use crate::common::get_lines;
use crate::Args;

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day09-test")
    } else {
        get_lines("day09")
    };

    let next_in_sequence_sum = sum_sequences(&lines, get_next_in_sequence);
    let prev_in_sequence_sum = sum_sequences(&lines, get_prev_in_sequence);

    (
        next_in_sequence_sum.to_string(),
        prev_in_sequence_sum.to_string(),
    )
}

fn sum_sequences(lines: &Vec<String>, pick_strategy: fn(Vec<isize>) -> isize) -> isize {
    lines
        .iter()
        .map(|x| {
            x.split(" ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(pick_strategy)
        .sum()
}

fn get_next_in_sequence(sequence: Vec<isize>) -> isize {
    let diffs = build_diff_chains(sequence);

    diffs.iter().map(|d| d.last().unwrap()).sum::<isize>()
}

fn get_prev_in_sequence(sequence: Vec<isize>) -> isize {
    let diffs = build_diff_chains(sequence);

    diffs
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let first_num = *d.first().unwrap();
            if i % 2 == 0 {
                first_num
            } else {
                -first_num
            }
        })
        .sum::<isize>()
}

fn build_diff_chains(sequence: Vec<isize>) -> Vec<Vec<isize>> {
    let mut diffs = Vec::new();
    let mut next_diff = sequence;

    while !is_zero_vector(&next_diff) {
        diffs.push(next_diff.clone());

        next_diff = pairwise_diffs(next_diff);
    }

    diffs
}

fn pairwise_diffs(sequence: Vec<isize>) -> Vec<isize> {
    sequence
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, el)| el - sequence.get(index - 1).unwrap())
        .collect::<Vec<isize>>()
}

fn is_zero_vector(sequence: &Vec<isize>) -> bool {
    sequence.iter().filter(|&&x| x != 0).count() == 0
}
