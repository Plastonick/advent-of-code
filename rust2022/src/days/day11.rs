use num::integer::lcm;

use crate::common::get_file_contents;

#[derive(Clone)]
struct Monkey {
    operation: u8,
    operand: usize,
    test: usize,
    if_true: usize,
    if_false: usize,
}

pub fn run(_: bool) {
    let file = get_file_contents("day11");

    let monkeys: Vec<Monkey> = file.split("\n\n").map(build_monkey).collect();
    let items: Vec<Vec<usize>> = file.split("\n\n").map(build_items).collect();

    run_for_part(1, &monkeys, &items);
    run_for_part(2, &monkeys, &items);
}

fn run_for_part(part: u8, monkeys: &Vec<Monkey>, items: &Vec<Vec<usize>>) {
    let mut mutable_items = items.clone();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    let rounds = if part == 1 { 20 } else { 10_000 };

    for _ in 0..rounds {
        (inspections, mutable_items) = iterate(&monkeys, mutable_items, inspections, part);
    }

    announce(rounds, part, &inspections);
}

fn iterate(
    monkeys: &Vec<Monkey>,
    starting_items: Vec<Vec<usize>>,
    inspections: Vec<usize>,
    part: u8,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut mutable_inspections = inspections.clone();
    let mut mutable_items = starting_items.clone();
    let mut index = 0;

    let common = monkeys
        .iter()
        .map(|x| x.test)
        .reduce(|x, y| lcm(x, y))
        .unwrap();

    for i in 0..mutable_inspections.len() {
        let monkey = &monkeys[i];
        let items = mutable_items[i].clone();
        let num_items = items.len();

        for item in items {
            let mut worry_level = match monkey.operation {
                0 => item + monkey.operand,
                1 => item * monkey.operand,
                2 => item * item,
                _ => {
                    println!("Oops! Shouldn't have gotten here...");
                    1
                }
            };

            worry_level = if part == 1 {
                worry_level / 3
            } else {
                worry_level - common * (worry_level / common)
            };

            let monkey_idx = if worry_level % monkey.test == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };

            mutable_items[monkey_idx].push(worry_level);
        }

        mutable_items[index] = Vec::new();
        mutable_inspections[index] += num_items;

        index += 1;
    }

    (mutable_inspections, mutable_items)
}

fn build_monkey(monkey_str: &str) -> Monkey {
    let lines: Vec<&str> = monkey_str.split('\n').collect();

    let operation = if lines[2].contains('+') {
        0
    } else if lines[2].contains("old * old") {
        2
    } else {
        1
    };
    let operand = if operation != 2 {
        lines[2][25..].parse::<usize>().unwrap()
    } else {
        0
    };

    let test = lines[3][21..].parse::<usize>().unwrap();
    let if_true = lines[4][29..].parse::<usize>().unwrap();
    let if_false = lines[5][30..].parse::<usize>().unwrap();

    Monkey {
        operation,
        operand,
        test,
        if_true,
        if_false,
    }
}

fn build_items(monkey_str: &str) -> Vec<usize> {
    let lines: Vec<&str> = monkey_str.split('\n').collect();

    let starting_items: Vec<usize> = lines[1][18..]
        .split(", ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    starting_items
}

fn announce(rounds: usize, part: u8, inspections: &Vec<usize>) {
    let mut mutable_inspections = inspections.clone();

    mutable_inspections.sort();
    mutable_inspections.reverse();

    println!(
        "Day 11, Part {}: The level of monkey business after {} rounds is {}",
        part,
        rounds,
        mutable_inspections[0] * mutable_inspections[1]
    );
}
