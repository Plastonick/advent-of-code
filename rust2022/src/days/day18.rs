use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::common::get_lines;

pub fn run(_: bool) {
    let lines = get_lines("day18");

    let points = lines
        .iter()
        .map(|row| {
            let point = row
                .split(',')
                .take(3)
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let [x, y, z] = <[isize; 3]>::try_from(point).ok().unwrap();

            (x, y, z)
        })
        .collect::<HashSet<_>>();

    println!(
        "Day 18, Part 1: The total exposed surface area of the cubes is {}",
        surface_area_of_points(&points)
    );

    println!(
        "Day 18, Part 2: The total internal volume of the cubes is {}",
        internal_area_of_points(&points)
    );
}

fn internal_area_of_points(points: &HashSet<(isize, isize, isize)>) -> usize {
    // assume points form a single larger piece
    // work out its volume + number of blocks

    // work out area of convex hull essentially
    // we can define _outside_ as anything beyond the limits of the boundary set
    // need to find a point _inside_ the convex hull, and then just path map limited by the hull, find volume

    let mut total_surface_area = surface_area_of_points(&points);
    let mut volume_points: HashSet<(isize, isize, isize)> = HashSet::new();
    let mut outside = HashSet::new();
    let mut limits = [[isize::MAX, isize::MIN]; 3];

    for (x, y, z) in points {
        let [x_limits, y_limits, z_limits] = limits;

        limits = [
            [min(x_limits[0], *x), max(x_limits[1], *x)],
            [min(y_limits[0], *y), max(y_limits[1], *y)],
            [min(z_limits[0], *z), max(z_limits[1], *z)],
        ];
    }

    for point in points {
        for n in get_neighbours(point) {
            if volume_points.contains(&n) {
                continue;
            }

            if let Some(volume) = map_volume(n, points, &mut outside, &limits) {
                total_surface_area -= surface_area_of_points(&volume);

                volume_points.extend(&volume);
            }
        }
    }

    total_surface_area
}

/// returns a vector of contiguous points incl. start, bounded by the boundary, or None if linked to (0, 0, 0)
fn map_volume(
    start: (isize, isize, isize),
    boundary: &HashSet<(isize, isize, isize)>,
    outside: &mut HashSet<(isize, isize, isize)>,
    limits: &[[isize; 2]; 3],
) -> Option<HashSet<(isize, isize, isize)>> {
    // trivially return None if we're starting at a boundary point
    if boundary.contains(&start) {
        return None;
    }

    let mut wave = vec![start];
    let mut volume = HashSet::from_iter(wave.iter().copied());

    while !wave.is_empty() {
        let mut next_wave = Vec::new();
        for point in &wave {
            // get neighbours
            let neighbours = get_neighbours(point);
            for n in neighbours {
                // we must have started from outside, update the known outside points and return None
                if is_outside(&n, limits, outside) {
                    outside.extend(volume);

                    return None;
                }

                // we've already included this in our volume! Continue
                if volume.contains(&n) {
                    continue;
                }

                // this marks part of our boundary, do not include it
                if boundary.contains(&n) {
                    continue;
                }

                // this point is part of our contiguous set, include it!
                volume.insert(n);
                next_wave.push(n);
            }
        }

        wave = next_wave;
    }

    Some(volume)
}

fn is_outside(
    point: &(isize, isize, isize),
    limits: &[[isize; 2]; 3],
    outside: &HashSet<(isize, isize, isize)>,
) -> bool {
    if outside.contains(&point) {
        return true;
    }

    if point.0 < limits[0][0] {
        return true;
    }

    if point.0 > limits[0][1] {
        return true;
    }

    if point.1 < limits[1][0] {
        return true;
    }

    if point.1 > limits[1][1] {
        return true;
    }

    if point.2 < limits[2][0] {
        return true;
    }

    if point.2 > limits[2][1] {
        return true;
    }

    return false;
}

fn surface_area_of_points(points: &HashSet<(isize, isize, isize)>) -> usize {
    let mut total_sa = points.len() * 6;

    for p in points {
        for neighbour in get_neighbours(p) {
            if points.contains(&neighbour) {
                total_sa -= 1;
            }
        }
    }

    total_sa
}

fn get_neighbours(point: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let mut neighbours = Vec::new();

    for x in [-1, 1] {
        let n = (point.0 + x, point.1, point.2);

        neighbours.push(n);
    }

    for y in [-1, 1] {
        let n = (point.0, point.1 + y, point.2);

        neighbours.push(n);
    }

    for z in [-1, 1] {
        let n = (point.0, point.1, point.2 + z);

        neighbours.push(n);
    }

    neighbours
}
