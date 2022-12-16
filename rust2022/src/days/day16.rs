use std::{cmp::max, collections::HashMap};

use regex::Regex;

use crate::common::get_lines;

#[derive(Debug)]
struct Valve {
    name: String,
    rate: isize,
    leads_to: Vec<String>,
}

impl Valve {
    pub fn from_str(string: &str) -> Valve {
        let pattern: Regex = Regex::new(
            r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (\w+)(?:, (\w+))*$",
        )
        .unwrap();

        let captures = pattern.captures(string).unwrap();
        let name = String::from(captures.get(1).map(|m| m.as_str()).unwrap());
        let rate = captures
            .get(2)
            .map(|m| m.as_str().parse::<isize>().unwrap())
            .unwrap();

        let mut index = 3;
        let mut leads_to = Vec::new();
        while let Some(lead) = captures.get(index).map(|m| m.as_str()) {
            leads_to.push(String::from(lead));
            index += 1;
        }

        Valve {
            name,
            rate,
            leads_to,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    position: String,
    ttl: isize,
    open: Vec<String>,
}

pub fn run(_: bool) {
    let lines = get_lines("day16-test");
    let valves: HashMap<String, Valve> = lines
        .iter()
        .map(|x| {
            let valve = Valve::from_str(x);
            (valve.name.to_owned(), valve)
        })
        .collect();
    let time_to_eruption = 30;

    // let mut cache = HashMap::new();
    // let best_value = get_best_value(
    //     (
    //         State {
    //             position: String::from("AA"),
    //             ttl: time_to_eruption,
    //             open: Vec::new(),
    //         },
    //         0,
    //     ),
    //     &valves,
    //     &mut cache,
    // );

    let start = State {
        position: String::from("AA"),
        ttl: time_to_eruption,
        open: Vec::new(),
    };
    // let mut states = Vec::new();
    // states.push(start);

    // let mut best = 0;
    let mut cache = HashMap::new();

    // loop {
    //     for (state, released) in states.pop() {
    //         let time_expired = state.ttl <= 0;
    //         let all_valves_open = state.open.len() == valves.len();
    //         let best_possible = 50; // TODO

    //         if time_expired || all_valves_open || best_possible < best {
    //             best = released;

    //             cache.insert(state, released);

    //             continue;
    //         }

    //         let at_valve = valves.get(&state.position).unwrap();

    //         // can we open the current valve?
    //         if !state.open.contains(&state.position) {
    //             let mut now_open = state.open.clone(); // TODO clone is bad mkay
    //             now_open.push(at_valve.name.clone()); // TODO clone is bad mkay
    //             now_open.sort();

    //             // try opening the current state
    //             let try_state = State {
    //                 open: now_open,
    //                 ttl: state.ttl - 1,
    //                 position: state.position.clone(),
    //             };

    //             states.push((try_state, released + (at_valve.rate * state.ttl)));
    //         }

    //         // can we go to a different valve?
    //         for valve_name in &at_valve.leads_to {
    //             let try_state = State {
    //                 position: valve_name.clone(),
    //                 ttl: state.ttl - 1,
    //                 open: state.open.clone(), // TODO clone is bad mkay
    //             };

    //             states.push((try_state, released));
    //         }
    //     }

    //     // when we've not got any more states to look over
    //     if states.len() == 0 {
    //         break;
    //     }
    // }

    let best = get_future_value(start, &valves, &mut cache);

    println!("Cache size: {}", cache.len());

    println!(
        "Day 16, Part 1: The most I can release in {} minutes is {}",
        time_to_eruption, best
    );
}

fn get_future_value(
    state: State,
    valves: &HashMap<String, Valve>,
    value_cache: &mut HashMap<State, isize>,
) -> isize {
    // if we've already been in this state before, return how valuable it was
    if let Some(cache_value) = value_cache.get(&state) {
        return cache_value.to_owned();
    }

    let time_expired = state.ttl <= 0;
    let all_valves_open = state.open.len() == valves.len();

    if time_expired || all_valves_open {
        // the volcano's erupted, we can't improve, I hope I've done enough!

        return 0;
    }

    // enumerate the possible actions and see what the most valuable action is
    let at_valve = valves.get(&state.position).unwrap();
    let mut best_increase = 0;

    // can we open the current valve?
    if !state.open.contains(&state.position) {
        let mut now_open = state.open.clone(); // TODO clone is bad mkay
        now_open.push(at_valve.name.clone()); // TODO clone is bad mkay
        now_open.sort();

        // try opening the current state
        let try_state = State {
            open: now_open,
            ttl: state.ttl - 1,
            position: state.position.clone(),
        };

        let future_value = get_future_value(try_state, valves, value_cache);
        let valve_open_value = at_valve.rate * state.ttl;
        best_increase = max(best_increase, valve_open_value + future_value);
    }

    // can we go to a different valve?
    for valve_name in &at_valve.leads_to {
        let try_state = State {
            position: valve_name.clone(),
            ttl: state.ttl - 1,
            open: state.open.clone(), // TODO clone is bad mkay
        };

        let future_value = get_future_value(try_state, valves, value_cache);
        best_increase = max(best_increase, future_value);
    }

    value_cache.insert(state, best_increase);

    best_increase
}
