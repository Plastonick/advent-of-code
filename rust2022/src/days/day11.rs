use crate::common::get_file_contents;

#[derive(Clone)]
struct Monkey {
    operation: u8,
    operand: usize,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

pub fn run() {
    let file = get_file_contents("day11");

    let monkeys: Vec<Monkey> = file.split("\n\n").map(build_monkey).collect();
    let items: Vec<Vec<usize>> = file.split("\n\n").map(build_items).collect();

    part1(&monkeys, &items);
    part2(&monkeys, &items);
}

fn part1(monkeys: &Vec<Monkey>, items: &Vec<Vec<usize>>) {
    let mut mut_monkeys = monkeys.clone();
    let mut mut_items = items.clone();

    let rounds = 20;

    for _ in 0..rounds {
        (mut_monkeys, mut_items) = iterate(mut_monkeys, mut_items, 1);
    }

    announce(rounds, 1, &mut_monkeys);
}

fn part2(monkeys: &Vec<Monkey>, items: &Vec<Vec<usize>>) {
    let mut mut_monkeys = monkeys.clone();
    let mut mut_items = items.clone();

    let rounds = 10000;

    for _ in 0..rounds {
        (mut_monkeys, mut_items) = iterate(mut_monkeys, mut_items, 2);
    }

    announce(rounds, 1, &mut_monkeys);
}

fn iterate(
    monkeys: Vec<Monkey>,
    starting_items: Vec<Vec<usize>>,
    part: u8,
) -> (Vec<Monkey>, Vec<Vec<usize>>) {
    let mut new_monkeys = monkeys.clone();
    let mut new_items = starting_items.clone();
    let mut index = 0;

    let common = monkeys.iter().map(|x| x.test).reduce(|x, y| x * y).unwrap();

    for i in 0..new_monkeys.len() {
        let monkey = new_monkeys[i].clone();
        let items = new_items[i].clone();
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

            let mut new_starting_items = new_items[monkey_idx].clone();
            new_starting_items.push(worry_level);

            new_items[monkey_idx] = new_starting_items;
            new_monkeys[monkey_idx] = Monkey {
                operation: new_monkeys[monkey_idx].operation,
                ..new_monkeys[monkey_idx]
            };
        }

        new_items[index] = Vec::new();
        new_monkeys[index] = Monkey {
            inspections: monkey.inspections + num_items,
            ..new_monkeys[index]
        };

        index += 1;
    }

    (new_monkeys, new_items)
}

fn build_monkey(monkey_str: &str) -> Monkey {
    let lines: Vec<&str> = monkey_str.split('\n').collect();

    let operation = if lines[2].contains('+') {
        0
    } else if lines[2].contains("old * old") {
        let line = lines[2].clone();
        println!("{}", String::from(line));
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
        inspections: 0,
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

fn announce(rounds: usize, part: u8, monkeys: &Vec<Monkey>) {
    let mut new_monkeys = monkeys.clone();

    new_monkeys.sort_by(|x, y| y.inspections.cmp(&x.inspections));

    println!(
        "Day 11, Part {}: The level of monkey business after {} rounds is {}",
        part,
        rounds,
        new_monkeys[0].inspections * new_monkeys[1].inspections
    );
}
