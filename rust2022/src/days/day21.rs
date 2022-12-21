use crate::{common::get_lines, Args};
use std::collections::{HashMap, VecDeque};

pub fn run(args: &Args) -> (String, String) {
    let lines = if args.test {
        get_lines("day21-test")
    } else {
        get_lines("day21")
    }
    .iter()
    .map(|x| x.to_owned())
    .collect::<VecDeque<_>>();

    let (part1, _, _) = solve(&mut lines.clone(), None);

    let shout_out = find_value(&lines, isize::MIN / 256, isize::MAX / 256);

    if !args.no_answers {
        println!("Day 21, Part 1: Root shouts out {}", part1);
        println!("Day 21, Part 1: The human should shout out {}", shout_out);
    }

    (part1.to_string(), shout_out.to_string())
}

fn find_value(lines: &VecDeque<String>, min: isize, max: isize) -> isize {
    let mut min = min;
    let mut max = max;

    let (_, left_min, right_min) = solve(&mut lines.clone(), Some(min));
    let (_, left_max, right_max) = solve(&mut lines.clone(), Some(max));

    // figure out if our function is in ascending or descending order, and if our right or left value is fixed
    let (direction, target_right) = if right_min == right_max {
        let diff = left_max - left_min;
        (diff / diff.abs(), true)
    } else {
        let diff = right_max - right_min;
        (diff / diff.abs(), false)
    };

    while min <= max {
        let midpoint = (min + max) / 2;

        let (_, left, right) = solve(&mut lines.clone(), Some(midpoint));

        let target = if target_right { right } else { left };
        let comparison = if target_right { left } else { right };

        if comparison * direction < target * direction {
            min = midpoint + 1;
        } else if comparison * direction > target * direction {
            max = midpoint - 1;
        } else {
            return midpoint;
        }
    }

    panic!("Failed to find target for human to shout");
}

fn solve(lines: &mut VecDeque<String>, test_value: Option<isize>) -> (isize, isize, isize) {
    let mut monkey_values = HashMap::new();

    // iterate over the monkeys, grabbing the monkeys that we can
    loop {
        while let Some(line) = lines.pop_front() {
            let parts = line.split(' ').collect::<Vec<_>>();

            match parts.len() {
                // second part is a number, add it to monkey_values and continue
                2 => {
                    let name = String::from(parts[0].strip_suffix(':').unwrap());
                    let number = if name == "humn" {
                        if let Some(value) = test_value {
                            value
                        } else {
                            parts[1].parse::<isize>().unwrap()
                        }
                    } else {
                        parts[1].parse::<isize>().unwrap()
                    };

                    monkey_values.insert(name, number);
                }
                // this monkey needs the values from two other monkeys!
                4 => {
                    let name = parts[0].strip_suffix(':').unwrap().to_string();
                    let monkey_one = parts[1].to_string();
                    let monkey_two = parts[3].to_string();

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

                        if name == "root" {
                            return (value, *operand_one.unwrap(), *operand_two.unwrap());
                        }

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
    }
}
