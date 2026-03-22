use crate::common::{get_lines, Answer};
use crate::Args;
use std::collections::{HashSet, VecDeque};

type Brick = (Vec<u32>, Vec<u32>);

pub fn run(_args: &Args) -> Answer {
    let lines = if _args.test {
        get_lines("day22-test")
    } else {
        get_lines("day22")
    };

    let mut falling_bricks = lines
        .iter()
        .filter_map(|x| x.split_once('~'))
        .map(|(a, b)| (parse_end(a), parse_end(b)))
        .map(|(a, b)| {
            (
                // sort into all mins then all max, to avoid lots of min/max further down
                vec![a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])],
                vec![a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])],
            )
        })
        .collect::<Vec<_>>();

    // sort the bricks by closeness to the ground!
    falling_bricks.sort_by(|a, b| a.0[2].cmp(&b.0[2]));

    let mut falling_bricks = VecDeque::from(falling_bricks);

    // then we'll drop each of the bricks as far as they'll go one by one
    let mut dropped_bricks = Vec::new();
    while let Some(brick) = falling_bricks.pop_front() {
        let settled = drop_next_brick(brick, &dropped_bricks);

        dropped_bricks.push(settled);
    }

    let (depends_on, resting_on) = build_dependency_tree(&dropped_bricks);
    let part_2 = (0..dropped_bricks.len())
        .map(|idx| (disintegration_value(idx, &depends_on, &resting_on).len() - 1) as u32)
        .sum::<u32>();

    // assume they're safe to disintegrate
    let mut safe_to_disintegrate = vec![true; dropped_bricks.len()];
    for (_, rests_on) in dropped_bricks {
        if rests_on.len() == 1 {
            // until we find an example where a brick rests on _only_ that brick
            safe_to_disintegrate[rests_on[0]] = false;
        }
    }

    (
        safe_to_disintegrate
            .iter()
            .filter(|x| **x)
            .count()
            .to_string(),
        part_2.to_string(),
    )
}

fn build_dependency_tree(
    dropped_bricks: &Vec<(Brick, Vec<usize>)>,
) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut depends_on = vec![HashSet::new(); dropped_bricks.len()];
    let mut resting_on = Vec::with_capacity(dropped_bricks.len());

    for (idx, (_, rests_on)) in dropped_bricks.iter().enumerate() {
        resting_on.push(HashSet::from_iter(rests_on.clone().into_iter()));

        for r_idx in rests_on {
            depends_on[*r_idx].insert(idx);
        }
    }

    (depends_on, resting_on)
}

fn disintegration_value(
    brick_idx: usize,
    depends_on: &Vec<HashSet<usize>>,
    resting_on: &Vec<HashSet<usize>>,
) -> HashSet<usize> {
    let mut disintegrated_bricks = HashSet::from_iter([brick_idx]);

    // push all the bricks that depend on our target brick into the wobblies
    let mut wobbly = VecDeque::from_iter(&depends_on[brick_idx]);
    while let Some(&idx) = wobbly.pop_front() {
        if resting_on[idx].is_subset(&disintegrated_bricks) {
            // this one bites the dust too!
            disintegrated_bricks.insert(idx);

            // and the ones that depend on it become wobbly!
            for r_idx in &depends_on[idx] {
                wobbly.push_back(r_idx);
            }
        }
    }

    disintegrated_bricks
}

fn drop_next_brick(brick: Brick, settled: &Vec<(Brick, Vec<usize>)>) -> (Brick, Vec<usize>) {
    let overlapping = settled
        .iter()
        .enumerate()
        .filter(|(_, (other, _))| {
            let x_overlap = brick.0[0] <= other.1[0] && brick.1[0] >= other.0[0];
            let y_overlap = brick.0[1] <= other.1[1] && brick.1[1] >= other.0[1];

            x_overlap && y_overlap
        })
        .collect::<Vec<_>>();

    // get the max-z of all the overlapping bricks, this will be where our brick ultimately rests
    let falls_to_z_index = overlapping
        .iter()
        .map(|(_, (x, _))| x.1[2] + 1)
        .max()
        .unwrap_or(1);

    // our brick will ultimately rest on the _top_ overlapping brick, but it may also rest on others too if they extend
    // far enough.
    let mut rests_on = Vec::new();
    for (idx, (overlaps, _)) in overlapping {
        if overlaps.1[2] == falls_to_z_index - 1 {
            rests_on.push(idx);
        }
    }

    let z_height = brick.1[2] - brick.0[2];

    (
        (
            vec![brick.0[0], brick.0[1], falls_to_z_index],
            vec![brick.1[0], brick.1[1], falls_to_z_index + z_height],
        ),
        rests_on,
    )
}

fn parse_end(str: &str) -> Vec<u32> {
    let (a, b) = str.split_once(',').expect("failed to split brick string");
    let (b, c) = b.split_once(',').expect("failed to split brick string");

    vec![
        a.parse::<u32>().unwrap(),
        b.parse::<u32>().unwrap(),
        c.parse::<u32>().unwrap(),
    ]
}
