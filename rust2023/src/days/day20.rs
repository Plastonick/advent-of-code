use crate::common::{get_lines, Answer};
use crate::Args;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

pub fn run(_args: &Args) -> Answer {
    let lines = if _args.test {
        get_lines("day20-test")
    } else {
        get_lines("day20")
    };

    let inverses = build_inverses(&lines);
    let flip_flops = build_modules(&lines, Some('%'));
    let conjunctions = build_modules(&lines, Some('&'))
        .into_iter()
        .map(|(address, outputs)| {
            (
                address.clone(),
                (outputs, inverses.get(&address).unwrap().to_owned()),
            )
        })
        .collect::<HashMap<String, (Vec<String>, Vec<String>)>>();
    let broadcasters = build_broadcasters(&lines);

    let mut flip_flop_states = flip_flops
        .keys()
        .map(|x| (x.to_owned(), 0))
        .collect::<HashMap<String, u8>>();
    let mut conjunction_states = conjunctions
        .iter()
        .map(|(address, (_, inputs))| {
            (
                address.to_owned(),
                inputs
                    .iter()
                    .map(|x| (x.to_owned(), 0))
                    .collect::<HashMap<String, u8>>(),
            )
        })
        .collect::<HashMap<String, HashMap<String, u8>>>();

    let mut pulses = [0, 0];
    let mut index = 0;
    let mut value_after_1000 = 0;

    let mut zg_inputs = inverses
        .get("zg")
        .unwrap()
        .iter()
        .map(|x| (x, vec![]))
        .collect::<HashMap<&String, Vec<usize>>>();

    'main: loop {
        index += 1;

        // send some pulses!
        // each pulse is (destination, origin, signal), signal: 0 => low, 1 => high
        let mut queue = VecDeque::from([("broadcaster".to_owned(), "button".to_owned(), 0)]);

        while let Some((address, origin, signal)) = queue.pop_front() {
            pulses[signal as usize] += 1;

            if signal == 1 {
                if let Some(visits) = zg_inputs.get_mut(&origin) {
                    visits.push(index);

                    dbg!(&zg_inputs);
                }
            }

            // have we got enough data for a naive period?
            if zg_inputs.values().filter(|x| x.len() < 2).count() == 0 {
                break 'main;
            }

            if address == "rx" {
                if signal == 0 {
                    println!("low signal rx found on index {index}");
                } else {
                    // println!("high signal rx found on index {index}");
                }
            }

            // println!(
            //     "{} -{}-> {}",
            //     origin,
            //     if signal == 0 { "low" } else { "high" },
            //     address
            // );

            let (outputs, pulse) = if let Some(outputs) = broadcasters.get(&address) {
                (outputs, signal)
            } else if let Some(outputs) = flip_flops.get(&address) {
                // Flip-flop modules (prefix %) are either on or off; they are initially off.

                // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                if signal == 1 {
                    continue;
                }

                // However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
                let new_pulse = (flip_flop_states.get(&address).unwrap() + 1) % 2;
                flip_flop_states.insert(address.to_owned(), new_pulse);

                (outputs, new_pulse)
            } else if let Some((outputs, _)) = conjunctions.get(&address) {
                // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input.

                // When a pulse is received, the conjunction module first updates its memory for that input
                let state = conjunction_states.get_mut(&address).unwrap();
                state.insert(origin, signal);

                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                let low_pulse_count = state.values().filter(|&&x| x == 0).count();
                let new_pulse = if low_pulse_count == 0 { 0 } else { 1 };

                (outputs, new_pulse)
            } else {
                // this is the rx signal
                if signal == 0 {
                    break;
                } else {
                    continue;
                }
            };

            for dest in outputs {
                queue.push_back((dest.to_owned(), address.to_owned(), pulse));
            }
        }

        if index == 1000 {
            value_after_1000 = pulses.iter().product::<usize>();

            if _args.test {
                break 'main;
            }
        }
    }

    let overall_period = zg_inputs
        .values()
        .map(|x| x[1] - x[0])
        .fold(1, |mut acc, a| {
            acc = lcm(acc, a);
            acc
        });

    (value_after_1000.to_string(), overall_period.to_string())
}

fn build_broadcasters(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    lines
        .iter()
        .filter(|x| x.len() > 11 && &x[..11] == "broadcaster")
        .map(|x| {
            let (_, output) = x.split_once(" -> ").unwrap();

            (
                "broadcaster".to_owned(),
                output
                    .split(", ")
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>()
}

fn build_modules(lines: &Vec<String>, prefix: Option<char>) -> HashMap<String, Vec<String>> {
    lines
        .iter()
        .filter(|x| {
            if let Some(prefix) = prefix {
                x.chars().next().unwrap() == prefix
            } else {
                true
            }
        })
        .map(|x| {
            let (input, output) = x.split_once(" -> ").unwrap();

            (
                input.trim_start_matches(['%', '&'].as_slice()).to_string(),
                output
                    .split(", ")
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>()
}

// list each output that links to a given input
fn build_inverses(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    build_modules(&lines, None)
        .iter()
        .fold(HashMap::new(), |mut acc, (input, outputs)| {
            for output in outputs {
                acc.entry(output.to_owned())
                    .or_insert_with(Vec::new)
                    .push(input.to_owned());
            }

            acc
        })
}
