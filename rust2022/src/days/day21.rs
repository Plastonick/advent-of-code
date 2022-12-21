use std::{
    collections::{HashMap, VecDeque},
    fmt::format,
};

use crate::{common::get_lines, Args};

pub fn run(args: &Args) -> (String, String) {
    let mut lines = if args.test {
        get_lines("day21-test")
    } else {
        get_lines("day21")
    }
    .iter()
    .map(|x| x.to_owned())
    .collect::<VecDeque<_>>();

    // let part1 = part1(&mut lines.clone());
    let part2 = part2(&mut lines.clone());

    ("".to_string(), "".to_string())
}

fn part2(lines: &mut VecDeque<String>) -> isize {
    let mut monkey_values = HashMap::new();

    loop {
        while let Some(line) = lines.pop_front() {
            let parts = line.split(' ').collect::<Vec<_>>();

            // todo handle root and humn cases

            match parts.len() {
                // second part is a number, add it to monkey_values and continue
                2 => {
                    let name = String::from(parts[0].strip_suffix(':').unwrap());
                    let number = if name == "humn" {
                        "x".to_string()
                    } else {
                        parts[1].to_string()
                    };

                    monkey_values.insert(name, number);
                }
                // this monkey needs the values from two other monkeys!
                4 => {
                    let name = String::from(parts[0].strip_suffix(':').unwrap());
                    let monkey_one = String::from(parts[1].to_owned());
                    let monkey_two = String::from(parts[3].to_owned());

                    let operand_one = monkey_values.get(&monkey_one);
                    let operand_two = monkey_values.get(&monkey_two);

                    let operation = if name == "root" { "=" } else { parts[2] };

                    if operand_one.is_some() && operand_two.is_some() {
                        // we know the values! Insert into the list and continue

                        let operation = format!(
                            "({}){}({})",
                            operand_one.unwrap(),
                            operation,
                            operand_two.unwrap()
                        );

                        monkey_values.insert(name, operation);
                    } else {
                        // add the monkey to the end of the queue to try again
                        lines.push_back(line);
                    }
                }
                _ => {
                    panic!("I didn't expect this!")
                }
            }
        }

        if let Some(value) = monkey_values.get("root") {
            println!("This is the root equation: {}", value);

            panic!();
        }
    }
}

fn part1(lines: &mut VecDeque<String>) -> isize {
    let mut monkey_values = HashMap::new();

    // iterate over the monkeys, grabbing the monkeys that we can
    loop {
        while let Some(line) = lines.pop_front() {
            let parts = line.split(' ').collect::<Vec<_>>();

            match parts.len() {
                // second part is a number, add it to monkey_values and continue
                2 => {
                    let name = String::from(parts[0].strip_suffix(':').unwrap());
                    let number = parts[1].parse::<isize>().unwrap();

                    monkey_values.insert(name, number);
                }
                // this monkey needs the values from two other monkeys!
                4 => {
                    let name = String::from(parts[0].strip_suffix(':').unwrap());
                    let monkey_one = String::from(parts[1].to_owned());
                    let monkey_two = String::from(parts[3].to_owned());

                    let operand_one = monkey_values.get(&monkey_one);
                    let operand_two = monkey_values.get(&monkey_two);

                    if operand_one.is_some() && operand_two.is_some() {
                        // we know the values! Insert into the list and continue
                        let value = match parts[2] {
                            "+" => operand_one.unwrap() + operand_two.unwrap(),
                            "-" => operand_one.unwrap() - operand_two.unwrap(),
                            "/" => operand_one.unwrap() / operand_two.unwrap(),
                            "*" => operand_one.unwrap() * operand_two.unwrap(),
                            _ => {
                                panic!("Unexpected operation!")
                            }
                        };

                        monkey_values.insert(name, value);
                    } else {
                        // add the monkey to the end of the queue to try again
                        lines.push_back(line);
                    }
                }
                _ => {
                    panic!("I didn't expect this!")
                }
            }
        }

        if let Some(value) = monkey_values.get("root") {
            return *value;
        }
    }
}
