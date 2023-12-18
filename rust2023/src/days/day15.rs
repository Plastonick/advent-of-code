use crate::common::{get_file_contents, Answer};
use crate::Args;
use std::collections::HashMap;

pub fn run(args: &Args) -> Answer {
    let file = if args.test {
        get_file_contents("day15-test")
    } else {
        get_file_contents("day15")
    };

    let hash_sum: usize = file.trim().split(",").map(hash).sum();

    let boxes = generate_boxes(file);

    let ordered_boxes = boxes
        .iter()
        .map(|b| {
            let mut vec = b.values().collect::<Vec<_>>();
            vec.sort_by(|(_, a), (_, b)| a.cmp(b));

            vec.iter().map(|(lens, _)| *lens).collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let focusing_power: usize = ordered_boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            (i + 1)
                * lenses
                    .iter()
                    .enumerate()
                    .map(|(slot, l)| (slot + 1) * l)
                    .sum::<usize>()
        })
        .sum();

    (hash_sum.to_string(), focusing_power.to_string())
}

fn generate_boxes(file: String) -> [HashMap<String, (usize, usize)>; 256] {
    let mut boxes: [HashMap<String, (usize, usize)>; 256] =
        core::array::from_fn(|_| HashMap::new());

    for (i, step) in file.trim().split(",").enumerate() {
        if step.contains('-') {
            let (label, _) = step.split_once('-').unwrap();
            let box_index = hash(label);

            boxes[box_index].remove(label);
        } else {
            let (label, focal_length_str) = step.split_once('=').unwrap();
            let box_index = hash(label);
            let focal_length: usize = focal_length_str.parse().unwrap();

            let order = if let Some((_, order)) = boxes[box_index].get(label) {
                *order
            } else {
                i
            };

            boxes[box_index].insert(label.to_string(), (focal_length, order));

            println!("{} => {}", label, box_index);
        }
    }

    boxes
}

fn hash(str: &str) -> usize {
    str.as_bytes()
        .iter()
        .fold(0, |a: u8, &b| a.wrapping_add(b).wrapping_mul(17)) as usize
}
