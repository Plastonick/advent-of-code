use crate::common::get_lines;
use crate::Args;

pub fn run(args: &Args) -> (String, String) {
    let (lines, part2_multiplier) = if args.test {
        (get_lines("day11-test"), 100 - 1)
    } else {
        (get_lines("day11"), 1_000_000 - 1)
    };

    let map = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let horz_gaps = map
        .iter()
        .enumerate()
        .filter(|(_, r)| is_empty_row(r))
        .map(|(r, _)| r)
        .collect::<Vec<_>>();

    let vert_gaps = (0..map[0].len())
        .filter(|&x| is_empty_column(x, &map))
        .collect::<Vec<_>>();

    let galaxies = map
        .iter()
        .enumerate()
        .map(|(r, row)| {
            grab_galaxies(&row)
                .iter()
                .map(|&c| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let distance_sum: usize = galaxies
        .iter()
        .map(|a| {
            galaxies
                .iter()
                .map(|b| dist(&a, &b, &horz_gaps, &vert_gaps, 1))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2;

    let distance_sum_2: usize = galaxies
        .iter()
        .map(|a| {
            galaxies
                .iter()
                .map(|b| dist(&a, &b, &horz_gaps, &vert_gaps, part2_multiplier))
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2;

    (distance_sum.to_string(), distance_sum_2.to_string())
}

fn is_empty_row(row: &Vec<char>) -> bool {
    row.iter().filter(|&&x| x != '.').count() == 0
}

fn is_empty_column(col: usize, map: &Vec<Vec<char>>) -> bool {
    map.iter()
        .map(|x| x.get(col).unwrap())
        .filter(|&&x| x != '.')
        .count()
        == 0
}

fn grab_galaxies(row: &Vec<char>) -> Vec<usize> {
    row.iter()
        .enumerate()
        .filter(|(_, &space)| space == '#')
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
}

fn dist(
    a: &(usize, usize),
    b: &(usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    multiplier: usize,
) -> usize {
    let (max_row, min_row) = (a.0.max(b.0), a.0.min(b.0));
    let (max_col, min_col) = (a.1.max(b.1), a.1.min(b.1));

    let horz_gaps_crossed = empty_rows
        .iter()
        .filter(|&&x| min_row <= x && x <= max_row)
        .count();
    let vert_gaps_crossed = empty_cols
        .iter()
        .filter(|&&x| min_col <= x && x <= max_col)
        .count();

    if a == &(2, 0) && b == &(6, 9) {
        dbg!(horz_gaps_crossed, vert_gaps_crossed);
    }

    (max_row - min_row)
        + (max_col - min_col)
        + ((horz_gaps_crossed + vert_gaps_crossed) * (multiplier))
}
