use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

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

#[derive(Debug)]
struct State {
    position: String,
    ttl: isize,
    open: HashSet<String>,
    released: isize,
}

impl State {
    fn cache_key(&self) -> String {
        let mut key = format!("{}-", self.position);

        for valve in &self.open {
            key += &format!(".{}", valve);
        }

        key += &format!("_{}", self.ttl);

        String::from(key)
    }
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

    // let mut states = vec![State {
    //     position: String::from("AA"),
    //     ttl: time_to_eruption,
    //     open: HashSet::new(),
    //     released: 0,
    // }];

    // let mut end_states = Vec::new();

    // loop {
    //     for state in states.pop() {
    //         if state.ttl <= 0 {
    //             // the volcano's already erupted, I hope I've done enough!

    //             end_states.push(state);
    //             continue;
    //         }

    //         let at_valve = valves.get(&state.position).unwrap();

    //         // can we open the current valve?
    //         if !state.open.contains(&state.position) {
    //             let mut now_open = state.open.clone(); // TODO clone is bad mkay
    //             now_open.insert(at_valve.name.clone()); // TODO clone is bad mkay

    //             // try opening the current state
    //             states.push(State {
    //                 released: state.released + (at_valve.rate * state.ttl),
    //                 open: now_open,
    //                 ttl: state.ttl - 1,
    //                 position: state.position,
    //             })
    //         }

    //         // can we go to a different valve?
    //         for valve_name in &at_valve.leads_to {
    //             states.push(State {
    //                 position: valve_name.clone(),
    //                 ttl: state.ttl - 1,
    //                 open: state.open.clone(), // TODO clone is bad mkay
    //                 released: state.released,
    //             })
    //         }
    //     }

    //     if states.len() == 0 {
    //         break;
    //     }
    // }

    let mut cache = HashMap::new();
    let best_value = get_best_value(
        State {
            position: String::from("AA"),
            ttl: time_to_eruption,
            open: HashSet::new(),
            released: 0,
        },
        &valves,
        &mut cache,
    );

    println!(
        "Day 16, Part 1: The most I can release in {} minutes is {}",
        time_to_eruption, best_value
    );
}

fn get_best_value(
    state: State,
    valves: &HashMap<String, Valve>,
    best_cache: &mut HashMap<String, isize>,
) -> isize {
    let key = state.cache_key();

    if let Some(cache_value) = best_cache.get(&key) {
        return cache_value.to_owned();
    }

    if state.ttl <= 0 {
        // the volcano's already erupted, I hope I've done enough!

        best_cache.insert(key, state.released);

        return state.released;
    }

    let at_valve = valves.get(&state.position).unwrap();
    let mut best_value = 0;

    // can we open the current valve?
    if !state.open.contains(&state.position) {
        let mut now_open = state.open.clone(); // TODO clone is bad mkay
        now_open.insert(at_valve.name.clone()); // TODO clone is bad mkay

        // try opening the current state
        let try_state = State {
            released: state.released + (at_valve.rate * state.ttl),
            open: now_open,
            ttl: state.ttl - 1,
            position: state.position,
        };

        best_value = max(best_value, get_best_value(try_state, valves, best_cache));
    }

    // can we go to a different valve?
    for valve_name in &at_valve.leads_to {
        let try_state = State {
            position: valve_name.clone(),
            ttl: state.ttl - 1,
            open: state.open.clone(), // TODO clone is bad mkay
            released: state.released,
        };

        best_value = max(best_value, get_best_value(try_state, valves, best_cache));
    }

    best_cache.insert(key, best_value);

    best_value
}
