use crate::common::get_file_contents;

#[derive(Clone)]
struct Monkey {
    starting_items: Vec<usize>,
    operation: u8,
    operand: usize,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

pub fn run() {
    let file = get_file_contents("day11");

    let mut monkeys: Vec<Monkey> = file.split("\n\n").map(build_monkey).collect();

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            monkeys = iterate(monkeys, index);
        }
    }

    monkeys.sort_by(|x, y| y.inspections.cmp(&x.inspections));

    println!(
        "Day 11, Part 1: The level of monkey business after 20 rounds is {}",
        monkeys[0].inspections * monkeys[1].inspections
    )
}

fn iterate(monkeys: Vec<Monkey>, index: usize) -> Vec<Monkey> {
    let mut new_monkeys: Vec<Monkey> = monkeys.clone();
    let monkey = monkeys[index].clone();
    let num_items = monkey.starting_items.len();

    for item in monkey.starting_items {
        let worry_level = match monkey.operation {
            0 => item + monkey.operand,
            1 => item * monkey.operand,
            2 => item * item,
            _ => 1,
        } / 3;

        let monkey_idx = if worry_level % monkey.test == 0 {
            monkey.if_true
        } else {
            monkey.if_false
        };

        let mut starting_items = new_monkeys[monkey_idx].starting_items.clone();
        starting_items.push(worry_level);

        new_monkeys[monkey_idx] = Monkey {
            starting_items,
            operation: new_monkeys[monkey_idx].operation.clone(),
            ..new_monkeys[monkey_idx]
        };
    }

    new_monkeys[index] = Monkey {
        starting_items: Vec::new(),
        inspections: monkey.inspections + num_items,
        ..new_monkeys[index]
    };

    new_monkeys
}

fn build_monkey(monkey_str: &str) -> Monkey {
    let lines: Vec<&str> = monkey_str.split('\n').collect();

    let starting_items: Vec<usize> = lines[1][18..]
        .split(", ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

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
        starting_items,
        operation,
        operand,
        test,
        if_true,
        if_false,
        inspections: 0,
    }
}
