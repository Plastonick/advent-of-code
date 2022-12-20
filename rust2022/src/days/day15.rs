use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::{common::get_lines, Args};
use regex::{Captures, Regex};

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Sensor {
    fn radius(&self) -> isize {
        self.distance_to(&self.closest_beacon)
    }

    fn distance_to(&self, other: &Point) -> isize {
        let x_diff = self.pos.x.abs_diff(other.x);
        let y_diff = self.pos.y.abs_diff(other.y);

        (x_diff + y_diff) as isize
    }

    fn wrapping(&self) -> [Line; 4] {
        let corners = [
            Point {
                x: self.pos.x - self.radius() - 1,
                y: self.pos.y,
            },
            Point {
                x: self.pos.x,
                y: self.pos.y + self.radius() + 1,
            },
            Point {
                x: self.pos.x + self.radius() + 1,
                y: self.pos.y,
            },
            Point {
                x: self.pos.x,
                y: self.pos.y - self.radius() - 1,
            },
        ];

        [
            Line {
                start: Point { ..corners[0] },
                end: Point { ..corners[1] },
            },
            Line {
                start: Point { ..corners[1] },
                end: Point { ..corners[2] },
            },
            Line {
                start: Point { ..corners[2] },
                end: Point { ..corners[3] },
            },
            Line {
                start: Point { ..corners[3] },
                end: Point { ..corners[0] },
            },
        ]
    }

    fn contains(&self, point: &Point) -> bool {
        let radius = self.radius();
        let distance = self.pos.distance_to(&point);

        radius >= distance
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> isize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as isize
    }
}

impl Line {
    /// returns Some(point) if both lines intersect at a point, else None
    fn intersect_point(&self, other: &Line) -> Option<Point> {
        if self.gradient() == other.gradient() {
            return None;
        }

        let grad_delta = self.gradient() - other.gradient();
        let y_intercept_delta = other.y_intersect() - self.y_intersect();
        let x_point = y_intercept_delta / grad_delta;
        let y_point = (self.gradient() * x_point) + self.y_intersect();

        Some(Point {
            x: x_point,
            y: y_point,
        })
    }

    fn gradient(&self) -> isize {
        if self.end.x - self.start.x == 0 {
            1
        } else {
            (self.end.y - self.start.y) / (self.end.x - self.start.x)
        }
    }

    fn y_intersect(&self) -> isize {
        self.start.y - (self.gradient() * self.start.x)
    }
}

pub fn run(args: &Args) -> (String, String) {
    let (lines, target_row, boundary) = if args.test {
        (get_lines("day15-test"), 10, 20)
    } else {
        (get_lines("day15"), 2_000_000, 4_000_000)
    };

    let sensors: Vec<_> = lines.iter().map(sensor_from_str).collect();
    let mut ignored_ranges: Vec<(isize, isize)> = Vec::new();
    let mut beacons_on_target = HashSet::new();
    let mut lines: Vec<Line> = Vec::new();

    // add lines to incl the four courners. It's possible our beacon exists
    // on the very edge of the boundary which isn't an intersection point of
    // two other sensor's buffer zones.
    lines.push(Line {
        start: Point { x: 0, y: 0 },
        end: Point {
            x: boundary,
            y: boundary,
        },
    });
    lines.push(Line {
        start: Point { x: 0, y: boundary },
        end: Point { x: boundary, y: 0 },
    });

    // the beacon _should_ exist at the boundary of a sensor, so find all the
    // boundaries, then find where two boundaries intersect at the search boundary
    for sensor in &sensors {
        for line in sensor.wrapping() {
            lines.push(line);
        }
    }

    let mut intersect_points: HashSet<Point> = HashSet::new();
    // now, our beacon will exist at the boundary of two lines
    // find all of these points which exist in our boundary
    for a in &lines {
        for b in &lines {
            if let Some(point) = a.intersect_point(b) {
                if is_in_boundary(&point, boundary) {
                    intersect_points.insert(point);
                }
            }
        }
    }

    let mut beacon_point = None;
    for point in &intersect_points {
        let mut contained = false;
        for sensor in &sensors {
            if sensor.contains(&point) {
                contained = true;
                break;
            }
        }

        if !contained {
            beacon_point = Some(point);
            break;
        }
    }

    for sensor in &sensors {
        if sensor.closest_beacon.y == target_row {
            beacons_on_target.insert(sensor.closest_beacon.x);
        }

        let dist_to_target = sensor.pos.y.abs_diff(target_row);
        let radius_at_target = sensor.radius() - dist_to_target as isize;

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
        } else {
            ignored_ranges.push(new_range);
        }
    }

    let mut part_1 = 0;
    for range in ignored_ranges {
        part_1 += 1 + range.1 - range.0;
    }

    if !args.no_answers {
        println!(
            "Day 15, Part 1: There are {} confirmed ignored positions on y={}",
            part_1 - beacons_on_target.len() as isize,
            target_row
        );
        if let Some(part_2_point) = beacon_point {
            println!(
                "Day 15, Part 2: The beacon's frequency is {}",
                (part_2_point.x * 4_000_000) + part_2_point.y,
            );
        }
    }

    ("".to_string(), "".to_string())
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

fn is_in_boundary(point: &Point, bound: isize) -> bool {
    if point.x < 0 {
        return false;
    }

    if point.x > bound {
        return false;
    }

    if point.y < 0 {
        return false;
    }

    if point.y > bound {
        return false;
    }

    true
}
