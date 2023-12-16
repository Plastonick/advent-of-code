use crate::common::{get_file_contents, transpose, Answer};
use crate::Args;

pub fn run(args: &Args) -> Answer {
    let content = if args.test {
        get_file_contents("day13-test")
    } else {
        get_file_contents("day13")
    };

    let patterns = content
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|l| l.chars().collect::<Vec<_>>())
                .filter(|l| l.len() > 0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mirror_score_sum: usize = patterns.iter().map(|p| mirror_score(p, 0)).sum();
    let fudged_score_sum: usize = patterns.iter().map(|p| mirror_score(p, 1)).sum();

    (mirror_score_sum.to_string(), fudged_score_sum.to_string())
}

fn mirror_score(pattern: &Vec<Vec<char>>, fudge_factor: usize) -> usize {
    if let Some(row) = find_horz_reflection(&pattern, fudge_factor) {
        return row * 100;
    }

    if let Some(col) = find_vert_reflection(&pattern, fudge_factor) {
        return col;
    }

    panic!("Expected to find a reflection!")
}

// a is "above" b, a and b are not necessarily same length.
fn are_mirrored(a: &[Vec<char>], b: &[Vec<char>], fudge_factor: usize) -> bool {
    let num_rows_checked = a.len().min(b.len());

    let fudginess = (0..num_rows_checked)
        .map(|i| {
            let j = a.len() - i - 1;

            let a_row = a.get(j).unwrap();
            let b_row = b.get(i).unwrap();

            similarity(&a_row, &b_row)
        })
        .sum::<usize>();

    fudginess == fudge_factor
}

fn find_horz_reflection(pattern: &Vec<Vec<char>>, fudge_factor: usize) -> Option<usize> {
    for i in 1..pattern.len() {
        if are_mirrored(&pattern[0..i], &pattern[i..pattern.len()], fudge_factor) {
            return Some(i);
        }
    }

    None
}

fn find_vert_reflection(pattern: &Vec<Vec<char>>, fudge_factor: usize) -> Option<usize> {
    find_horz_reflection(&transpose(pattern.clone()), fudge_factor)
}

fn similarity(a: &Vec<char>, b: &Vec<char>) -> usize {
    assert_eq!(a.len(), b.len());

    a.iter()
        .enumerate()
        .filter(|(i, a)| b.get(*i).unwrap() != *a)
        .count()
}
