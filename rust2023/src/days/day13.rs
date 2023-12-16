use crate::common::{get_file_contents, Answer};
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

    let mirror_score_sum: usize = patterns.iter().map(mirror_score).sum();

    (mirror_score_sum.to_string(), "TODO".to_string())
}

fn mirror_score(pattern: &Vec<Vec<char>>) -> usize {
    if let Some(row) = find_horz_reflection(&pattern) {
        return row * 100;
    }

    if let Some(col) = find_vert_reflection(&pattern) {
        return col;
    }

    panic!("Expected to find a reflection!")
}

// a is "above" b, a and b are not necessarily same length.
fn are_mirrored(a: &[Vec<char>], b: &[Vec<char>]) -> bool {
    let num_rows_checked = a.len().min(b.len());

    for i in 0..num_rows_checked {
        let j = a.len() - i - 1;

        let a_row = a.get(j).unwrap().iter().collect::<String>();
        let b_row = b.get(i).unwrap().iter().collect::<String>();

        if a_row != b_row {
            return false;
        }
    }

    true
}

fn find_horz_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    for i in 1..pattern.len() {
        if are_mirrored(&pattern[0..i], &pattern[i..pattern.len()]) {
            return Some(i);
        }
    }

    None
}

fn find_vert_reflection(pattern: &Vec<Vec<char>>) -> Option<usize> {
    find_horz_reflection(&transpose2(pattern.clone()))
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
