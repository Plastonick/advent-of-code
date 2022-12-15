use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::common::get_lines;
use regex::{Captures, Regex};

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

pub fn run(_: bool) {
    let lines = get_lines("day15");
    let target_row = 2_000_000;
    let sensors: Vec<_> = lines.iter().map(sensor_from_str).collect();
    let mut ignored_ranges: Vec<(isize, isize)> = Vec::new();
    let mut beacons_on_target = HashSet::new();

    for sensor in sensors {
        if sensor.closest_beacon.y == target_row {
            beacons_on_target.insert(sensor.closest_beacon.x);
        }

        let x_diff = sensor.pos.x.abs_diff(sensor.closest_beacon.x);
        let y_diff = sensor.pos.y.abs_diff(sensor.closest_beacon.y);
        let radius = x_diff + y_diff;
        let dist_to_target = sensor.pos.y.abs_diff(target_row);
        let radius_at_target = radius as isize - dist_to_target as isize;

        if radius_at_target < 0 {
            // this beacon's exclusitivity zone doesn't intersect target row
            continue;
        }

        let new_range = (
            sensor.pos.x - radius_at_target,
            sensor.pos.x + radius_at_target,
        );

        // find any existing ranges that intersect this
        let mut intersecting_ranges: Vec<_> = ignored_ranges
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, x)| x.1 >= new_range.0 && new_range.1 >= x.0)
            .collect();

        if intersecting_ranges.len() > 0 {
            // delete old ranges
            intersecting_ranges.sort_by(|(a, _), (b, _)| b.cmp(a));

            for (i, _) in &intersecting_ranges {
                ignored_ranges.remove(*i);
            }

            // add new union range
            let (_, mut union_range) = intersecting_ranges
                .iter()
                .cloned()
                .reduce(|(_, a), (_, b)| (0, (min(a.0, b.0), max(a.1, b.1))))
                .unwrap();

            union_range = (
                min(union_range.0, new_range.0),
                max(union_range.1, new_range.1),
            );

            ignored_ranges.push(union_range);
            // ignored_ranges.push(new_range);
        } else {
            ignored_ranges.push(new_range);
        }
    }

    let mut part_1 = 0;
    for range in ignored_ranges {
        part_1 += 1 + range.1 - range.0;
    }

    println!(
        "Day 15, Part 1: {}",
        part_1 - beacons_on_target.len() as isize
    );
}

fn sensor_from_str(string: &String) -> Sensor {
    fn get_capture(captures: &Captures, index: usize) -> isize {
        captures
            .get(index)
            .map(|m| m.as_str().parse::<isize>().unwrap())
            .unwrap()
    }

    let pattern: Regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)$").unwrap();

    let captures = pattern.captures(string).unwrap();

    let s_x = get_capture(&captures, 1);
    let s_y = get_capture(&captures, 2);
    let cb_x = get_capture(&captures, 3);
    let cb_y = get_capture(&captures, 4);

    Sensor {
        pos: Point { x: s_x, y: s_y },
        closest_beacon: Point { x: cb_x, y: cb_y },
    }
}
