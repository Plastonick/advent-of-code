use crate::common::{get_lines, Answer};
use crate::maps::Vector;
use crate::Args;
use pathfinding::prelude::dijkstra;
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

    let (_, part_1) = state_after_steps(start_point, &garden_plots, steps_part_1);

    let size = Vector {
        row: lines.len() as isize,
        col: lines.get(0).unwrap().len() as isize,
    };

    let part_2 = places_after_steps_3(&start_point, &garden_plots, &size, steps_2);

    (part_1.len().to_string(), part_2.to_string())
}

// fn places_after_steps(
//     start: &Vector,
//     garden_plots: &HashSet<Vector>,
//     size: &Vector,
//     steps: isize,
// ) -> isize {
//     // we need to know how many places can be visited after "steps" number of steps.
//     // garden_plot determines the map we exist in, and repeats infinitely in all directions.
//     // we start at "start" vector.
//
//     // we should initialise a hashset of virtual "garden plots".
//
//     // key will be the virtual plot address
//     // value will be the max number of steps remaining to get there
//     let mut virtual_plots = HashMap::new();
//
//     // each element of wave is (position, steps_remaining, virtual_plot)
//     let mut wave = vec![(start, steps, Vector { row: 0, col: 0 })];
//
//     while let Some((pos, steps_remaining, virtual_plot)) = wave.pop() {
//         let neighbour_directions = [
//             Vector { row: 1, col: 0 },  // down
//             Vector { row: -1, col: 0 }, // up
//             Vector { row: 0, col: -1 }, // left
//             Vector { row: 0, col: 1 },  // right
//         ];
//
//         for neighbour_direction in neighbour_directions {
//             // how long does it take to get to that neighbour?
//             let neighbour_plot = neighbour_direction.add(&virtual_plot);
//
//             let (pos, distance_to_neighbour) =
//                 distance(&pos, &neighbour_direction, &size, &garden_plots);
//
//             let distance_remaining = steps_remaining - distance_to_neighbour;
//
//             if distance_remaining <= 0 {
//                 continue;
//             }
//
//             // replace virtual plots entry if we have a greater distance remaining for that address
//             if let Some(existing_distance) = virtual_plots.get(&neighbour_plot) {
//                 if *existing_distance > distance_remaining {
//                     virtual_plots.insert(neighbour_plot, distance_remaining);
//                     wave.push((pos, distance_remaining, neighbour_plot));
//                 }
//             } else {
//                 virtual_plots.insert(neighbour_plot, distance_remaining);
//                 wave.push((pos, distance_remaining, neighbour_plot));
//             }
//         }
//     }
//
//     dbg!(virtual_plots);
//
//     4
// }

// this doesn't work....
fn places_after_steps_3(
    start: &Vector,
    garden_plots: &HashSet<Vector>,
    size: &Vector,
    steps: isize,
) -> isize {
    // confirm a few input assumptions
    assert_eq!(size.row, size.col);

    let corners = [
        Vector { row: 0, col: 0 },
        Vector {
            row: size.row - 1,
            col: 0,
        },
        Vector {
            row: 0,
            col: size.col - 1,
        },
        Vector {
            row: size.row - 1,
            col: size.col - 1,
        },
    ];

    let (_, even_saturated_map) =
        state_after_steps(start.clone(), &garden_plots, size.row * size.col * 2);
    let (_, odd_saturated_map) =
        state_after_steps(start.clone(), &garden_plots, 1 + (size.row * size.col * 2));
    let mut plots_visited = even_saturated_map.len() as isize;

    dbg!(even_saturated_map.len());
    dbg!(odd_saturated_map.len());

    for corner in corners {
        let opposite_corner = size.sub(&corner).sub(&Vector { row: 1, col: 1 });

        // distance to the corner, then 2 more to get to the next map
        let (_, steps_to_next_map) = distance(&start, &|x| x.eq(&corner), &garden_plots);
        let steps_remaining = steps - (steps_to_next_map + 2);

        dbg!(steps_remaining);

        // provide a number of steps guaranteed to saturate the map fully, we return as soon as
        let (steps_to_saturate, saturated_map) =
            state_after_steps(opposite_corner, &garden_plots, size.row * size.col * 2);

        let full_triangle_width = if steps_remaining > steps_to_saturate {
            1 + ((steps_remaining - steps_to_saturate) / size.row)
        } else {
            0
        };

        let (first_set, second_set) = striped_triangular(full_triangle_width);
        let remaining_steps = (steps_remaining - steps_to_saturate) % size.row;
        let (_, partial_map) = state_after_steps(opposite_corner, &garden_plots, remaining_steps);

        dbg!(remaining_steps, &full_triangle_width + 1, partial_map.len());
        println!();

        // TODO this _might_ be the wrong way around!
        let (odd_count, even_count) = if remaining_steps % 2 == 0 {
            (first_set, second_set)
        } else {
            (second_set, first_set)
        };

        plots_visited += odd_count * odd_saturated_map.len() as isize;
        plots_visited += even_count * even_saturated_map.len() as isize;
        plots_visited += (full_triangle_width + 1) * partial_map.len() as isize;
    }

    let steps_to_saturate = size.row + size.col - 2;

    // analysing the input, we see the orthogonal rays are actually quite simple, since there are paths going all the way directly to each edge
    let length_of_ray = ((steps - steps_to_saturate) - ((size.row + 1) / 2)) / size.row;
    let even_count = (length_of_ray + 1) / 2;
    let odd_count = length_of_ray - even_count;

    plots_visited += (odd_count * odd_saturated_map.len() as isize) * 4;
    plots_visited += (even_count * even_saturated_map.len() as isize) * 4;

    let remaining_steps = (steps - ((size.row + 1) / 2)) % size.row;

    dbg!(length_of_ray);
    dbg!(remaining_steps);

    for row in [1] {
        // distance from start to top
        //
    }

    // add the vertical/horizontal rays

    plots_visited
}

// returns a pair of split triangular numbers.
// the first value is the sum of all odd numbers from 1 to n
// the second value is the sum of all even numbers from 1 to n
fn striped_triangular(n: isize) -> (isize, isize) {
    let odd_pair = ((n + 1) / 2) * ((n + 1) / 2);
    let even_pair = ((n * (n + 1)) / 2) - odd_pair;

    (odd_pair, even_pair)
}

// static mut DISTANCE_CACHE: Option<HashMap<(Vector, Vector, Vector), (Vector, isize)>> = None;

fn distance<F>(from: &Vector, success: &F, garden_plots: &HashSet<Vector>) -> (Vec<Vector>, isize)
where
    F: Fn(&Vector) -> bool,
{
    dijkstra(
        from,
        |pos| {
            get_neighbours(pos, &garden_plots)
                .into_iter()
                .map(|n| (n, 1))
                .collect::<Vec<(Vector, isize)>>()
        },
        &success,
    )
    .unwrap()
}

// returns the minimum number of steps required, and the state after those steps
fn state_after_steps(
    start: Vector,
    garden_plots: &HashSet<Vector>,
    steps: isize,
) -> (isize, HashSet<Vector>) {
    let mut positions = HashSet::from([start]);
    let mut previous_count = None;

    for i in 0..steps {
        if (steps - i) % 2 == 0 {
            if let Some(count) = previous_count {
                if count == positions.len() {
                    return (i, positions);
                }
            }

            previous_count = Some(positions.len());
        }

        positions = step(positions, &garden_plots);
    }

    (steps, positions)
}

fn step(possibilities: HashSet<Vector>, garden_plots: &HashSet<Vector>) -> HashSet<Vector> {
    possibilities
        .into_iter()
        .map(|pos| get_neighbours(&pos, &garden_plots))
        .flatten()
        .collect::<HashSet<Vector>>()
}

fn get_neighbours(pos: &Vector, garden_plots: &HashSet<Vector>) -> Vec<Vector> {
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

// add a test case for `striped_triangular`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_striped_triangular() {
        assert_eq!(striped_triangular(1), (1, 0));
        assert_eq!(striped_triangular(2), (1, 2));
        assert_eq!(striped_triangular(3), (4, 2));
        assert_eq!(striped_triangular(4), (4, 6));
        assert_eq!(striped_triangular(5), (9, 6));
        assert_eq!(striped_triangular(6), (9, 12));
        assert_eq!(striped_triangular(7), (16, 12));
        assert_eq!(striped_triangular(8), (16, 20));
        assert_eq!(striped_triangular(9), (25, 20));
        assert_eq!(striped_triangular(10), (25, 30));
    }

    #[test]
    fn test_sum_striped_triangular() {
        let (odds, evens) = striped_triangular(10);

        assert_eq!(odds + evens, 55);
    }
}
