use crate::common::get_lines;
use crate::Args;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

type Nodes = HashMap<String, (String, String)>;

pub fn run(args: &Args) -> (String, String) {
    let (start_node, end_node) = ("AAA", "ZZZ");

    let lines = if args.test {
        get_lines("day08-test")
    } else {
        get_lines("day08")
    };

    let directions = lines.get(0).unwrap().chars().collect::<Vec<_>>();
    let nodes = lines.iter().skip(2).map(line_to_node).collect::<Nodes>();

    let mut part_1_goes = 0;
    let mut direction = directions.get(part_1_goes % directions.len()).unwrap();
    let mut current_node = start_node.to_string();

    loop {
        current_node = next_node(current_node, *direction, &nodes);
        part_1_goes += 1;

        if current_node == end_node.to_string() {
            break;
        }

        direction = directions.get(part_1_goes % directions.len()).unwrap();
    }

    let part_2 = nodes
        .iter()
        .map(|(n, _)| n.to_owned())
        .filter(|n| n.chars().nth(2).unwrap() == 'A')
        .map(|n| start_node_period(n.clone(), &directions, &nodes))
        .fold(1, |mut acc, int| {
            acc = lcm(acc, int);
            acc
        });

    (part_1_goes.to_string(), part_2.to_string())
}

fn next_node(current_node_address: String, direction: char, nodes: &Nodes) -> String {
    let current_node = nodes.get(&current_node_address).unwrap().to_owned();

    match direction {
        'L' => current_node.0,
        'R' => current_node.1,
        _ => {
            panic!("Uh oh! Unexpected direction!")
        }
    }
}

fn line_to_node(line: &String) -> (String, (String, String)) {
    let re = Regex::new(r"([A-Z\d]+)").unwrap();

    let matches = re
        .find_iter(line)
        .filter_map(|node| node.as_str().parse().ok())
        .collect::<Vec<String>>();

    (
        matches.get(0).unwrap().to_owned(),
        (
            matches.get(1).unwrap().to_owned(),
            matches.get(2).unwrap().to_owned(),
        ),
    )
}

// TODO: what's going on here? This shouldn't be correct... We should care how long it takes to hit the cycle, not just how long the cycle is, right?
fn start_node_period(start_node_address: String, directions: &Vec<char>, nodes: &Nodes) -> usize {
    let mut goes = 0;
    let mut current_address = start_node_address;
    let mut direction;

    loop {
        direction = *directions.get(goes % directions.len()).unwrap();
        current_address = next_node(current_address, direction, nodes);
        goes += 1;

        if current_address.chars().nth(2).unwrap() == 'Z' {
            return goes;
        }
    }
}
