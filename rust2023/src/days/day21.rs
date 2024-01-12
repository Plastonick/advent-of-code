use crate::common::{get_lines, Answer};
use crate::maps::{Vector, _print};
use crate::Args;
use std::collections::HashSet;

pub fn run(_args: &Args) -> Answer {
    let (lines, steps_part_1, steps_2) = if _args.test {
        (get_lines("day21-test"), 6, 5000)
    } else {
        (get_lines("day21"), 64, 26501365)
    };

    let mut start_point = Vector { row: 0, col: 0 };
    let garden_plots = lines
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if ch != '#' {
                        let pos = Vector {
                            row: row as isize,
                            col: col as isize,
                        };

                        if ch == 'S' {
                            start_point = pos;
                        }

                        Some(pos)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashSet<Vector>>();

    dbg!(&start_point);
    let first_locations = HashSet::from([start_point]);
    let part_1 = state_after_steps(first_locations, &garden_plots, steps_part_1);

    _print(&garden_plots);
    _print(&part_1);

    (part_1.len().to_string(), "".to_string())
}

fn state_after_steps(
    possibilities: HashSet<Vector>,
    garden_plots: &HashSet<Vector>,
    steps: usize,
) -> HashSet<Vector> {
    if steps == 0 {
        return possibilities;
    }

    let next_possibilities = possibilities
        .into_iter()
        .map(|pos| get_neighbours(pos, &garden_plots))
        .flatten()
        .collect::<HashSet<Vector>>();

    state_after_steps(next_possibilities, &garden_plots, steps - 1)
}

fn get_neighbours(pos: Vector, garden_plots: &HashSet<Vector>) -> Vec<Vector> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(row, col)| Vector { row, col })
        .filter_map(|d| {
            let new_pos = pos.add(&d);

            if garden_plots.contains(&new_pos) {
                Some(new_pos)
            } else {
                None
            }
        })
        .collect()
}
